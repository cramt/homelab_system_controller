use core::fmt::Display;

use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Duration;
use embedded_io_async::Read;
use log::*;
use picoserve::{
    extract::State,
    make_static,
    response::{Connection, IntoResponse, Response, ResponseWriter, StatusCode},
    routing::{get, post},
    Config, ResponseSent, Router, Timeouts,
};

use crate::pin_context::PinContext;

const WEB_TASK_POOL_SIZE: usize = 2;

pub async fn init_web(spawner: Spawner, stack: Stack<'static>, context: PinContext) {
    let state =
        InnerState(make_static!(Mutex<CriticalSectionRawMutex, PinContext>, Mutex::new(context)));
    for id in 0..WEB_TASK_POOL_SIZE {
        let _ = spawner.spawn(web_task(
            id,
            stack,
            Config::new(Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
            }),
            state,
        ));
    }
}

#[derive(Clone, Copy)]
struct InnerState(&'static Mutex<CriticalSectionRawMutex, PinContext>);

struct AppState {
    inner: InnerState,
}

impl picoserve::extract::FromRef<AppState> for InnerState {
    fn from_ref(state: &AppState) -> Self {
        state.inner
    }
}

pub struct CorsResponse<C: Display> {
    status: StatusCode,
    content: C,
}

impl<C: Display> IntoResponse for CorsResponse<C> {
    async fn write_to<R: Read, W: ResponseWriter<Error = R::Error>>(
        self,
        connection: Connection<'_, R>,
        response_writer: W,
    ) -> Result<ResponseSent, W::Error> {
        const CORS_HEADERS: [(&str, &str); 5] = [
            ("access-control-allow-headers", "*"),
            ("access-control-expose-headers", "*, Authorization"),
            ("access-control-allow-origin", "*"),
            ("access-control-allow-methods", "*"),
            ("access-control-allow-credentials", "*"),
        ];
        response_writer
            .write_response(
                connection,
                Response::new(self.status, format_args!("{}", self.content))
                    .with_headers(CORS_HEADERS),
            )
            .await
    }
}

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task(
    id: usize,
    stack: embassy_net::Stack<'static>,
    config: Config<Duration>,
    state: InnerState,
) -> ! {
    let port =
        konst::result::unwrap_ctx!(konst::primitive::parse_u16(env!("HARDWARE_OBSERVER_PORT")));
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    let router = Router::new()
        .route(
            "/get",
            get(|State(InnerState(inner)): State<InnerState>| async move {
                let mut i = inner.lock().await;
                let value = i.read_pc_led().await;
                info!("request");
                CorsResponse {
                    status: StatusCode::OK,
                    content: value.unwrap_or(0),
                }
            }),
        )
        .route(
            "/post",
            post(|State(InnerState(inner)): State<InnerState>| async move {
                let mut i = inner.lock().await;
                i.click_button().await;
                CorsResponse {
                    status: StatusCode::OK,
                    content: "",
                }
            }),
        );

    info!("web task {id} spawning");

    picoserve::listen_and_serve_with_state(
        id,
        &router,
        &config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
        &AppState { inner: state },
    )
    .await
}

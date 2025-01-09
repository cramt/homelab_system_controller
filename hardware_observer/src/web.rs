use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Duration;
use log::*;
use picoserve::{
    extract::State, make_static, response::DebugValue, routing::get, Config, Router, Timeouts,
};

const WEB_TASK_POOL_SIZE: usize = 2;

pub async fn init_web(spawner: Spawner, stack: Stack<'static>) {
    let state = InnerState(make_static!(Mutex<CriticalSectionRawMutex, u64>, Mutex::new(0)));
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
struct InnerState(&'static Mutex<CriticalSectionRawMutex, u64>);

struct AppState {
    inner: InnerState,
}

impl picoserve::extract::FromRef<AppState> for InnerState {
    fn from_ref(state: &AppState) -> Self {
        state.inner
    }
}

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task(
    id: usize,
    stack: embassy_net::Stack<'static>,
    config: Config<Duration>,
    state: InnerState,
) -> ! {
    let port = 1234;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    let router = Router::new().route(
        "/",
        get(|State(InnerState(inner)): State<InnerState>| async move {
            info!("request");
            DebugValue(inner)
        }),
    );

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

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use log::{Metadata, Record};
use postcard_rpc::server::Sender;

use crate::rpc::AppTx;

struct MyLogger(Channel<CriticalSectionRawMutex, &'static str, 8>);
impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(s) = record.args().as_str() {
                self.0.try_send(s).unwrap();
            }
        }
    }
    fn flush(&self) {}
}

static LOGGER: MyLogger = MyLogger(Channel::new());

#[embassy_executor::task]
pub async fn logging_task(sender: Sender<AppTx>) -> ! {
    unsafe {
        let _ = log::set_logger_racy(&LOGGER);
        log::set_max_level_racy(log::LevelFilter::Info);
    }

    loop {
        let _ = sender.log_str(LOGGER.0.receive().await).await;
    }
}

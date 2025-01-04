use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel, lazy_lock::LazyLock};
use log::{Metadata, Record};
use postcard_rpc::server::Sender;

use crate::AppTx;

static CHANNEL: LazyLock<Channel<NoopRawMutex, &'static str, 8>> = LazyLock::new(Channel::new);

pub const LOGGER: MyLogger = MyLogger;

pub struct MyLogger;
impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(str) = record.args().as_str() {
                let channel = CHANNEL.get();
                let _ = channel.try_send(str);
            }
        }
    }
    fn flush(&self) {}
}

#[embassy_executor::task]
pub async fn logging_task(sender: Sender<AppTx>) {
    let receiver = CHANNEL.get().receiver();
    loop {
        sender.log_str("bro_idk").await;
        let _ = sender.log_str(receiver.receive().await).await;
    }
}

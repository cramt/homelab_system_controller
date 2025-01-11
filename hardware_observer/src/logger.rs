use crate::rpc::AppTx;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Sender as ChannelSender},
};
use log::{Metadata, Record};
use postcard_rpc::server::Sender;
use serde_json_core::heapless::String;

struct MyLogger(Channel<CriticalSectionRawMutex, String<16>, 32>);
impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        struct ArgSender<'a>(ChannelSender<'a, CriticalSectionRawMutex, String<16>, 32>);
        impl core::fmt::Write for ArgSender<'_> {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                let mut i = 0;
                loop {
                    if (i * 16) > s.len() {
                        break;
                    }
                    let j = i + 1;
                    let mut str = String::new();
                    let _ = str.push_str(&s[(i * 16)..core::cmp::min(s.len(), j * 16)]);
                    self.0.try_send(str).unwrap();
                    i = j;
                }
                Ok(())
            }
        }
        if self.enabled(record.metadata()) {
            let mut writer = ArgSender(self.0.sender());
            let _ = core::fmt::write(&mut writer, format_args!("{}\n", record.args()));
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
        let str = LOGGER.0.receive().await;
        sender.log_str(str.as_str()).await.unwrap();
    }
}

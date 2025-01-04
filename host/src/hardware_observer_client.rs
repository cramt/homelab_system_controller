use std::convert::Infallible;

use common::{PingEndpoint, PRODUCT_ID, VENDOR_ID};
use postcard_rpc::{
    header::VarSeqKind,
    host_client::{HostClient, HostErr},
    standard_icd::{LoggingTopic, WireError, ERROR_PATH},
};

#[derive(Debug)]
pub enum HardwareObserverError<E> {
    Comms(HostErr<WireError>),
    Endpoint(E),
}

impl<E> From<HostErr<WireError>> for HardwareObserverError<E> {
    fn from(value: HostErr<WireError>) -> Self {
        Self::Comms(value)
    }
}

pub struct HardwareObserverClient {
    pub client: HostClient<WireError>,
}

impl HardwareObserverClient {
    pub fn new() -> Self {
        let client = HostClient::new_raw_nusb(
            |d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID,
            ERROR_PATH,
            8,
            VarSeqKind::Seq2,
        );
        Self { client }
    }

    pub async fn ping(&self, id: u64) -> Result<u64, HardwareObserverError<Infallible>> {
        let val = self.client.send_resp::<PingEndpoint>(&id).await?;
        Ok(val)
    }

    pub async fn logging_run(&self) {
        loop {
            if let Ok(mut logsub) = self.client.subscribe_multi::<LoggingTopic>(64).await {
                while let Ok(msg) = logsub.recv().await {
                    println!("LOG: {msg}");
                }
            }
        }
    }
}

impl Default for HardwareObserverClient {
    fn default() -> Self {
        Self::new()
    }
}

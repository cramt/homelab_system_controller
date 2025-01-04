use std::convert::Infallible;

use common::PingEndpoint;
use postcard_rpc::{
    header::VarSeqKind,
    host_client::{HostClient, HostErr},
    standard_icd::{WireError, ERROR_PATH},
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
            |d| d.product_string() == Some("homelab_system_controller_hardware_observer"),
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
}

impl Default for HardwareObserverClient {
    fn default() -> Self {
        Self::new()
    }
}

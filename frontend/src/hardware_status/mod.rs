pub mod status;

use dioxus::prelude::*;
use status::Status;

#[component]
pub fn HardwareStatus() -> Element {
    let domain = env!("BASE_DOMAIN");
    let port = env!("HARDWARE_OBSERVER_PORT");
    let url = format!("http://hardware_observer.{domain}:{port}");
    rsx! {
        Status {
            url: url
        }
    }
}

pub mod service_status;
use crate::assets::{
    bazarr::Bazarr, foundry::Foundry, jellyfin::Jellyfin, prowlarr::Prowlarr, qbit::Qbit,
    radarr::Radarr, servatrice::Servatrice, sonarr::Sonarr,
};
use dioxus::prelude::*;
use service_status::ServiceStatus;

#[component]
pub fn Services() -> Element {
    let domain = option_env!("BASE_DOMAIN").unwrap_or("localhost:1234");
    rsx! {
        div {
            p {
                class: "text-4xl text-center p-4",
                "service status"
            }
        }
        div {
            class: "flex flex-row flex-wrap gap-4 justify-center items-center content-center",
            ServiceStatus {
                url: format!("https://jellyfin.{domain}/"),
                name: "jellyfin",
                icon: rsx! { Jellyfin {}}
            }
            ServiceStatus {
                url: format!("https://sonarr.{domain}/"),
                name: "sonarr",
                icon: rsx! { Sonarr {}}
            }
            ServiceStatus {
                url: format!("https://radarr.{domain}/"),
                name: "radarr",
                icon: rsx! { Radarr {}}
            }
            ServiceStatus {
                url: format!("https://qbit.{domain}/"),
                name: "qbittorrent",
                icon: rsx! { Qbit {}}
            }
            ServiceStatus {
                url: format!("https://bazarr.{domain}/"),
                name: "bazarr",
                icon: rsx! { Bazarr {}}
            }
            ServiceStatus {
                url: format!("https://foundry-a.{domain}/"),
                name: "foundry A",
                icon: rsx! { Foundry {}}
            }
            ServiceStatus {
                url: format!("https://prowlarr.{domain}/"),
                name: "prowlarr",
                icon: rsx! { Prowlarr {}}
            }
            ServiceStatus {
                url: format!("wss://cockatrice.{domain}/servatrice"),
                name: "servatrice",
                icon: rsx! { Servatrice {}}
            }
        }
    }
}

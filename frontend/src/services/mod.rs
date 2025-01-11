pub mod service_status;
use crate::assets::{
    bazarr::Bazarr, foundry::Foundry, jellyfin::Jellyfin, prowlarr::Prowlarr, qbit::Qbit,
    radarr::Radarr, servatrice::Servatrice, sonarr::Sonarr,
};
use dioxus::prelude::*;
use service_status::ServiceStatus;

#[component]
pub fn Services() -> Element {
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
                url: "https://jellyfin.cramt.schniebster.dk/",
                name: "jellyfin",
                icon: rsx! { Jellyfin {}}
            }
            ServiceStatus {
                url: "https://sonarr.cramt.schniebster.dk/",
                name: "sonarr",
                icon: rsx! { Sonarr {}}
            }
            ServiceStatus {
                url: "https://radarr.cramt.schniebster.dk/",
                name: "radarr",
                icon: rsx! { Radarr {}}
            }
            ServiceStatus {
                url: "https://qbit.cramt.schniebster.dk/",
                name: "qbittorrent",
                icon: rsx! { Qbit {}}
            }
            ServiceStatus {
                url: "https://bazarr.cramt.schniebster.dk/",
                name: "bazarr",
                icon: rsx! { Bazarr {}}
            }
            ServiceStatus {
                url: "https://foundry-a.cramt.schniebster.dk/",
                name: "foundry A",
                icon: rsx! { Foundry {}}
            }
            ServiceStatus {
                url: "https://prowlarr.cramt.schniebster.dk/",
                name: "prowlarr",
                icon: rsx! { Prowlarr {}}
            }
            ServiceStatus {
                url: "wss://cockatrice.cramt.schniebster.dk/servatrice",
                name: "servatrice",
                icon: rsx! { Servatrice {}}
            }
        }
    }
}

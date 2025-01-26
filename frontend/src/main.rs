pub mod assets;
pub mod hardware_status;
pub mod services;

use crate::hardware_status::HardwareStatus;
use crate::services::Services;
use cmd_proc_macro::cmd_execute;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/services")]
    Services {},
    #[route("/hardware_status")]
    HardwareStatus {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let tailwind = std::str::from_utf8(cmd_execute!(
        "cd $CARGO_MANIFEST_DIR && npm run --silent build_css"
    ))
    .unwrap();
    rsx! {
        style {
            {tailwind}
        }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 {
                class: "text-4xl",
                Link {
                    to: Route::Services {},
                    "Service List"
                }
            }
            h1 {
                class: "text-4xl",
                Link {
                    to: Route::HardwareStatus {},
                    "Hardware Status"
                }
            }
        }
    }
}

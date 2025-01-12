pub mod assets;
pub mod services;

use crate::services::Services;
use cmd_proc_macro::cmd_execute;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/services")]
    Services {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let tailwind =
        std::str::from_utf8(cmd_execute!("cd $CARGO_MANIFEST_DIR && npm run build_css")).unwrap();
    rsx! {
        style {
            {tailwind}
        }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {}
}

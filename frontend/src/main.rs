pub mod service_status;

use dioxus::prelude::*;
use service_status::ServiceStatus;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        ServiceStatus { url: "https://jellyfin.cramt.schniebster.dk/" }
    }
}

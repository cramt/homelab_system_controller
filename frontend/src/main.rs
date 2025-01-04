use dioxus::prelude::*;
use futures::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::Message;

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
pub fn Hero() -> Element {
    let result = use_resource(move || async {
        let websocket = reqwest_websocket::websocket("http://localhost:8989/ws")
            .await
            .unwrap();
        let (mut sender, mut receiver) = websocket.split();
        sender
            .send(Message::Text("test from server".into()))
            .await
            .unwrap();
        loop {
            match receiver.try_next().await {
                Err(_) => {
                    break;
                }
                Ok(None) => {
                    break;
                }
                Ok(Some(Message::Text(text))) => return text,
                _ => {}
            }
        }
        "bro".to_string()
    });
    match &*result.read_unchecked() {
        Some(str) => {
            // if it is, render the stories
            rsx! {
                div {
                    "{str}"
                }
            }
        }
        _ => {
            rsx! {
                div {
                    "test"
                }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

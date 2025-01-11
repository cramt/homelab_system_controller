use crate::assets::{checkmark::Checkmark, error::Error, spinner::Spinner};
use dioxus::prelude::*;
use reqwest_websocket::RequestBuilderExt;

#[component]
pub fn ServiceStatus(url: String, icon: Element, name: String) -> Element {
    let new_url = url.clone();
    let result = use_resource(move || {
        let url = url.clone();
        async move {
            if url.starts_with("wss://") {
                async fn inner(url: &str) -> Option<bool> {
                    let client = reqwest::Client::builder().build().unwrap();
                    client
                        .get(url)
                        .upgrade()
                        .send()
                        .await
                        .ok()?
                        .into_websocket()
                        .await
                        .ok()?;
                    Some(true)
                }
                inner(url.as_str()).await.unwrap_or(false)
            } else {
                let client = reqwest::Client::builder().build().unwrap();
                match client.get(url.as_str()).send().await {
                    Ok(x) => !x.status().is_server_error(),
                    Err(_) => false,
                }
            }
        }
    });
    let text = match *result.read_unchecked() {
        Some(true) => {
            rsx! {
                Checkmark {}
            }
        }
        Some(false) => {
            rsx! {
                Error {}
            }
        }
        None => {
            rsx! {
                Spinner {}
            }
        }
    };

    rsx! {
        a {
            href: new_url,
            target: "about_blank",
            div {
                class: "
                    flex flex-col justify-center items-center content-center gap-2 flex-none
                    border-purple-800 rounded-lg border-8
                    p-4 min-w-36
                    ",
                span {
                    class: "text",
                    {name}
                }
                div {
                    class: "w-16 h-16",
                    {icon}
                }
                {text}
            }
        }
    }
}

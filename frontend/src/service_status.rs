use dioxus::prelude::*;

#[component]
pub fn ServiceStatus(url: String) -> Element {
    let result = use_resource(move || {
        let url = url.clone();
        async move {
            let client = reqwest::Client::new();
            match client.get(url.as_str()).send().await {
                Ok(x) => !x.status().is_server_error(),
                Err(_) => false,
            }
        }
    });
    match &*result.read_unchecked() {
        Some(success) => {
            let str = if *success { "success" } else { "failure" };
            rsx! {
                div {
                    "{str}"
                }
            }
        }
        _ => {
            rsx! {
                div {
                    "loading"
                }
            }
        }
    }
}

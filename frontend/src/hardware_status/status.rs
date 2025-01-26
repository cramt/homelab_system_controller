use dioxus::prelude::*;

#[component]
pub fn Status(url: String) -> Element {
    let result = use_resource(move || {
        let url = url.clone();
        async fn inner(url: String) -> Option<String> {
            reqwest::get(format!("{url}/get"))
                .await
                .ok()?
                .text()
                .await
                .ok()
        }
        async move { inner(url).await }
    });
    match &*result.read_unchecked() {
        None => rsx! {"loading"},
        Some(x) => rsx! {
            {x.clone()}
        },
    }
}

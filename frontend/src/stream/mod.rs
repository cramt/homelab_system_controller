use dioxus::prelude::*;
#[component]
pub fn Stream() -> Element {
    let stream_path = option_env!("STREAM_PATH").unwrap_or("ws://localhost:1234/stream");
    rsx! {
        canvas {
            id: "test",
        }
        script {
            src: "https://jsmpeg.com/jsmpeg.min.js",
        }
    }
}

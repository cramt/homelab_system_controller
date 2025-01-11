use dioxus::prelude::*;

#[component]
pub fn Jellyfin() -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 512 512",
            defs {
                linearGradient {
                    x1: "126.15",
                    x2: "457.68",
                    y2: "410.73",
                    "gradientUnits": "userSpaceOnUse",
                    y1: "219.32",
                    id: "color",
                    stop { "stop-color": "#aa5cc3", offset: "0%" }
                    stop { offset: "100%", "stop-color": "#00a4dc" }
                }
            }
            path {
                d: "M190.56 329.07c8.63 17.3 122.4 17.12 130.93 0 8.52-17.1-47.9-119.78-65.46-119.8-17.57 0-74.1 102.5-65.47 119.8z",
                fill: "url(#color)"
            }
            path {
                d: "M58.75 417.03c25.97 52.15 368.86 51.55 394.55 0S308.93 56.08 256.03 56.08c-52.92 0-223.25 308.8-197.28 360.95zm68.04-45.25c-17.02-34.17 94.6-236.5 129.26-236.5 34.67 0 146.1 202.7 129.26 236.5-16.83 33.8-241.5 34.17-258.52 0z",
                fill: "url(#color)"
            }
        }
    }
}

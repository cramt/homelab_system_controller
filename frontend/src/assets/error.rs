use dioxus::prelude::*;

#[component]
pub fn Error() -> Element {
    rsx! {
        svg {
            class: "w-6 h-6",
            "viewBox": "0 0 48 48",
            defs { id: "defs7",
                linearGradient { "osb:paint": "solid", id: "linearGradient828",
                    stop {
                        style: "stop-color:#ff0000;stop-opacity:1;",
                        offset: "0",
                        id: "stop826",
                    }
                }
                linearGradient {
                    x2: "0",
                    "gradientUnits": "userSpaceOnUse",
                    y1: "47.37",
                    y2: "-1.429",
                    id: "0",
                    stop { "stop-color": "#c52828", id: "stop2" }
                    stop { "stop-color": "#ff5454", offset: "1", id: "stop4" }
                }
            }
            g {
                "enable-background": "new",
                style: "fill-opacity:1",
                transform: "matrix(.99999 0 0 .99999-58.37.882)",
                id: "g13",
                circle {
                    cx: "82.37",
                    cy: "23.12",
                    fill: "url(#0)",
                    r: "24",
                    style: "fill-opacity:1;fill:#dd3333",
                    id: "circle9",
                }
                path {
                    d: "m87.77 23.725l5.939-5.939c.377-.372.566-.835.566-1.373 0-.54-.189-.997-.566-1.374l-2.747-2.747c-.377-.372-.835-.564-1.373-.564-.539 0-.997.186-1.374.564l-5.939 5.939-5.939-5.939c-.377-.372-.835-.564-1.374-.564-.539 0-.997.186-1.374.564l-2.748 2.747c-.377.378-.566.835-.566 1.374 0 .54.188.997.566 1.373l5.939 5.939-5.939 5.94c-.377.372-.566.835-.566 1.373 0 .54.188.997.566 1.373l2.748 2.747c.377.378.835.564 1.374.564.539 0 .997-.186 1.374-.564l5.939-5.939 5.94 5.939c.377.378.835.564 1.374.564.539 0 .997-.186 1.373-.564l2.747-2.747c.377-.372.566-.835.566-1.373 0-.54-.188-.997-.566-1.373l-5.939-5.94",
                    "fill-opacity": ".842",
                    style: "fill-opacity:1;fill:#ffffff",
                    fill: "#fff",
                    id: "path11",
                }
            }
        }
    }
}

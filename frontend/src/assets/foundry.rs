use dioxus::prelude::*;

#[component]
pub fn Foundry() -> Element {
    rsx! {
        // fuck this find a svg at some point, why the fuck these idiots not using svg
        img { src: "https://r2.foundryvtt.com/website-static-public/assets/icons/fvtt.png" }
    }
}

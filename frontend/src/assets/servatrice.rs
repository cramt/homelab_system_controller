use dioxus::prelude::*;

#[component]
pub fn Servatrice() -> Element {
    rsx! {
        // fuck this find a svg at some point, why the fuck these idiots not using svg
        img { src: "https://raw.githubusercontent.com/Cockatrice/cockatrice.github.io/master/images/cockatrice_logo.png" }
    }
}

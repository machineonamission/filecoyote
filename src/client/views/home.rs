use crate::client::dioxus_components::input::Input;
use super::super::components::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "p-4",
            InvisibleDrop {},
            Input {
                {"test"}
            },
            p {
                "Welcome to the Home Page!"
                Icon { "face" }
                "haha hehe hoho"
            }
        }
    }
}

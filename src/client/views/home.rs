use crate::client::dioxus_components::input::Input;
use super::super::components::invisible_drop::InvisibleDrop;
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
            p {"hiiiiii heujifrsudijfohnlesrtgh"}
        }
    }
}

use dioxus::prelude::*;
use crate::client::components::input::Input;

#[component]
pub fn Home() -> Element {
    rsx! {
        Input {
            {"test"}
        }
        p {"hiiiiii heujifrsudijfohnlesrtgh"}
    }
}

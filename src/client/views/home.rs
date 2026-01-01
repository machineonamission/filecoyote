use super::super::components::*;
use crate::client::dioxus_components::button::Button;
use crate::client::dioxus_components::input::Input;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let count = 50;
    let mut cover = use_signal(|| false);
    let mut fill = use_signal(|| true);
    let urls_vec: Vec<String> = (0..count)
        .map(|i| {
            format!(
                "https://picsum.photos/{}/{}?random={}",
                rand::random_range(100..=1000),
                rand::random_range(100..=1000),
                i
            )
        }) // Closure squares each number
        .collect();
    let urls = use_hook(|| urls_vec);
    rsx! {
        div {
            class: "p-4 min-h-screen",
            Button {
                onclick: move |_| {
                    cover.toggle();
                },
                "cover"
            },
            Button {
                onclick: move |_| {
                    fill.toggle();
                },
                "fill"
            },
            p {
                "cover: {cover()}, fill: {fill()}"
            }
            Input {
                {"test"}
            },
            InvisibleDrop {
                FlexGrid {
                    fill: fill,
                    for i in 0..count {
                        Thumbnail {
                            cover: cover,
                            src: urls[i].clone(),
                        }
                    }
                }
            }
        },
    }
}

use dioxus::prelude::*;
use dioxus_primitives::aspect_ratio::AspectRatio;

// thubmail

#[component]
pub fn Thumbnail(
    src: String,
    cover: Signal<bool>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {
            class: "h-32 w-full",
            class: if !cover() {
                "flex items-center justify-center"
            },
            img {
                class: "rounded-md",
                class: if cover() {
                    "w-full h-full object-cover"
                } else {
                    "w-auto h-auto max-w-full max-h-full"
                },
                src: "{src}",
                alt: "Thumbnail",
                ..attributes,
            }
        }
    }
}

/*
#[component]
pub fn Thumbnail(
    src: String,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {
            class: "w-32 h-32 place-items-center",
            img {
                class: "w-auto h-auto max-w-full max-h-full rounded-md",
                src: "{src}",
                alt: "Thumbnail",
                ..attributes,
            }
        }
    }
}
*/
use dioxus::prelude::*;
use super::Icon;

#[component]
pub fn InvisibleDrop(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
        div {
            class: "relative",
            ondragenter: move |ev| {
                // println!("ondragenter");
                prevent_defaults(&ev);
                visible.set(true);
            },
            ondragover: move |ev| {
                // println!("ondragover");
                prevent_defaults(&ev);
                visible.set(true);
            },
            ondragleave: move |ev| {
                // println!("ondragleave");
                prevent_defaults(&ev);
                visible.set(false);
            },
            ondrop: move |ev| {
                // println!("ondrop");
                prevent_defaults(&ev);
                visible.set(false);
            },
            ..attributes,
            div {
                class: "absolute inset-0 pointer-events-none",
                class: if visible() { "fadein" } else { "fadeout" },
                style: "opacity:0",
                div {
                    class: "sticky top-0",
                    style: "height: min(100vh, 100%);",
                    div {
                        class: "w-full h-full borderdance grid place-items-center",
                        style: "container-type: size;",
                        div {
                            style: "font-size: calc(min(6cqw, 18cqh));",
                            h1 {
                                class: "text-center font-bold text-[3em] leading-0",
                                Icon {
                                    "file_upload"
                                }
                            },
                            h3 {
                                class: "text-center font-bold",
                                "Drop files to upload"
                            }
                        }
                    }
                },
            },
            { children }
        }
    }
}

fn prevent_defaults(ev: &DragEvent) {
    ev.prevent_default();
    ev.stop_propagation();
}
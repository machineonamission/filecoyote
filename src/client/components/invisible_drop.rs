use dioxus::prelude::*;

#[component]
pub fn InvisibleDrop(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,

) -> Element {
    let mut visible = use_signal(|| false);
    rsx! {
        div {
            class: "invisible-drop-zone borderdance",
            class: if visible() { "fadein" } else { "fadeout" },
            style: "opacity:0",
            ondragenter: move |ev| {
                println!("ondragenter");
                prevent_defaults(&ev);
                visible.set(true);
                // Show highlight
            },
            ondragover: move |ev| {
                println!("ondragover");
                prevent_defaults(&ev);
                visible.set(true);
                // Show highlight
            },
            ondragleave: move |ev| {
                println!("ondragleave");
                prevent_defaults(&ev);
                visible.set(false);
                // Hide highlight
            },
            ondrop: move |ev| {
                println!("ondrop");
                prevent_defaults(&ev);
                visible.set(false);
                // Handle dropped files
                // Hide highlight
            },
            ..attributes,
            // Optionally render highlight
            // {highlight}
        }
    }
}

fn prevent_defaults(ev: &DragEvent) {
    ev.prevent_default();
    ev.stop_propagation();
}
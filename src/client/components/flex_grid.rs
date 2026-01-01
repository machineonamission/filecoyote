use dioxus::prelude::*;

#[component]
pub fn FlexGrid(
    fill: Signal<bool>,
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "grid",
            // style: "",
            style: if fill() {
                "gap: 10px;grid-template-columns: repeat(auto-fill, minmax(calc(var(--spacing) * 32), 1fr));"
            } else {
                "gap: 10px;grid-template-columns: repeat(auto-fill, calc(var(--spacing) * 32));"
            },
            ..attributes,
            {children}
        }
    }
}
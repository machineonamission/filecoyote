use dioxus::prelude::*;

#[component]
pub fn Icon(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        // document::Stylesheet {href: asset!("../assets/material-symbols/material-symbols.css") }
        span {
            class: "material-symbols",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn IconSheet(
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        style { {format!(
            include_str!("../assets/material-symbols/material-symbols.css"),
            asset!("../assets/material-symbols/MaterialSymbolsRounded[FILL,GRAD,opsz,wght].woff2")
        )} }
    }
}

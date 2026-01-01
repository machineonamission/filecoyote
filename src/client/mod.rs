use views::Home;
use dioxus::prelude::*;

mod views;
pub mod entry;
mod components;
mod ahl;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}


const STYLESHEETS: [Asset; 3] = [
    asset!("./assets/tailwind.css"),
    asset!("./components/dx-components-theme.css"),
    asset!("./css/style.css"),
];
const FAVICON: Asset = asset!("./assets/favicon.ico");

#[component]
pub fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        for sheet in STYLESHEETS {
            document::Stylesheet { href: sheet }
        }
        ahl::AhlPatch {}

        Router::<Route> {}
    }
}
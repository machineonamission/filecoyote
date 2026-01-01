use views::Home;
use dioxus::prelude::*;

mod views;
pub mod entry;
mod components;
mod dioxus_components;
mod ahl;

use components::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}


const STYLESHEETS: [Asset; 5] = [
    asset!("./assets/tailwind.css"),
    asset!("./dioxus_components/dx-components-theme.css"),
    asset!("./css/borderdance.css"),
    asset!("./css/fadeinout.css"),
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
        components::icon::IconSheet {}

        Router::<Route> {}
    }
}
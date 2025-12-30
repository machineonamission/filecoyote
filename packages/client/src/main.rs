use dioxus::prelude::*;
use views::{Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // dioxus's entrypoint is always the client, so we have to ask server to init.
    // including this manual serve call so init can be async
    
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        server::init().await;
        
        // default stuff to let this function continue as normal
        Ok(dioxus::server::router(App))
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
use dioxus::prelude::*;
mod client;
mod server;

fn main() {
    // dioxus's entrypoint is always the client, so we have to ask server to init.
    // including this manual serve call so init can be async

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        server::backend::init().await;

        // default stuff to let this function continue as normal
        Ok(dioxus::server::router(client::App))
    });
    //
    #[cfg(not(feature = "server"))]
    dioxus::launch(client::App);
}

use crate::{client, server};

// main() called only on the server
pub(crate) fn main() {
    dioxus::serve(|| async move {
        // init the server (mostly db), this is the best place to do that, though it could
        // technically go in just the main() func
        super::init().await;

        // default stuff to let this function continue as normal
        // and yeah we do need to import client but thats fine
        Ok(dioxus::server::router(client::App))
    });
}

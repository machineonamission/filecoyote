use crate::client;

// main() called only on the client
#[cfg(not(feature = "server"))]
pub(crate) fn main() {
    // let cfg = Config::new();
    dioxus::launch(client::App);
}
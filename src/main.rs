mod client;
mod server;

fn main() {
    #[cfg(feature = "server")]
    server::backend::entry::main();

    #[cfg(not(feature = "server"))]
    client::entry::main();
}

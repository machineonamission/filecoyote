use dioxus::prelude::*;

#[cfg(feature = "server")]
pub mod backend;

#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    backend::init().await;
    Ok(input)
}

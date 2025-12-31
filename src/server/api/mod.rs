use dioxus::prelude::*;


#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    super::backend::init().await;
    Ok(input)
}
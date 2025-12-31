use dioxus::fullstack::Lazy;
use dioxus::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use std::sync::{LazyLock, OnceLock};

mod db;

#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

async fn init_db() {}

pub async fn init() {
    println!("server init");
    // init the db
    DATABASE.get();
}

static DATABASE: Lazy<DatabaseConnection> = Lazy::new(|| async move {
    let db = Database::connect(
        r"sqlite://C:/Users/Melody/RustroverProjects/filecoyote/db.sqlite?mode=rwc",
    )
    .await?;
    // synchronizes database schema with entity definitions
    let reg = db.get_schema_registry("server::db::entity::*");
    reg.sync(&db).await?;
    dioxus::Ok(db)
});

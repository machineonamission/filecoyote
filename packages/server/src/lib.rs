use dioxus::fullstack::Lazy;
use dioxus::prelude::*;
use sea_orm::{Database, DatabaseConnection};

mod db;

#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

pub async fn init() {
    println!("server init");
    let db = Database::connect(
        r"sqlite://C:/Users/Melody/RustroverProjects/filecoyote/db.sqlite?mode=rwc",
    )
        .await.unwrap();
    // synchronizes database schema with entity definitions
    println!("{}", module_path!());
    let reg = db.get_schema_registry("server::db::entity::*");
    println!("{:?}", reg);
    reg
        .sync(&db)
        .await.unwrap();
}

static DATABASE: Lazy<DatabaseConnection> = Lazy::new(|| async move {
    let db = Database::connect(
        r"sqlite://C:/Users/Melody/RustroverProjects/filecoyote/db.sqlite?mode=rwc",
    )
    .await?;
    // synchronizes database schema with entity definitions
    println!("{}", module_path!());
    let reg = db.get_schema_registry("server::db::entity::*");
    println!("{:?}", reg);
    reg
        .sync(&db)
        .await?;
    dioxus::Ok(db)
});

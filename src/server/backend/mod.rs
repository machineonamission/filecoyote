pub mod db;
pub mod entry;

use dioxus::fullstack::Lazy;
use sea_orm::{Database, DatabaseConnection};

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
    db.get_schema_registry("server::db::entity::*")
        .sync(&db)
        .await?;
    dioxus::Ok(db)
});

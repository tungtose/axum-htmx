mod error;

pub use self::error::{Error, Result};

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::env;
use tracing::info;

pub type Db = Pool<Sqlite>;

pub async fn new_db_pool() -> Result<Db> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    info!("db_url: {:?}", db_url);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|ex| Error::FaillToCreatePool(ex.to_string()))
}

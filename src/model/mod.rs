pub mod error;
pub mod store;
pub mod tasks;

pub use self::error::{Error, Result};

use crate::model::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<ModelManager> {
        let db = new_db_pool().await?;

        Ok(ModelManager {db})
    }

    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}

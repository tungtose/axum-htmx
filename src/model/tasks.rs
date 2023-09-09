use crate::model::ModelManager;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{Error, Result};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub descriptions: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub descriptions: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub descriptions: Option<String>,
}

pub struct TaskBmc;

impl TaskBmc {
    pub async fn create(mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();

        let mut conn = db.acquire().await?;

        let id = sqlx::query!(
            r#"
            INSERT INTO todos ( descriptions ) VALUES ( ?1 )
        "#,
            task_c.descriptions,
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

        // let (id,) = sqlx::query_as::<_, (i64,)>(
        //     "INSERT INTO todos (descriptions) values ($1) returning id",
        // )
        // .bind(task_c.descriptions)
        // .fetch_one(db)
        // .await?;

        // TODO: need to bench the insert
        // std::thread::sleep(std::time::Duration::from_millis(50));

        Ok(id)
    }

    pub async fn get(mm: &ModelManager, id: i64) -> Result<Task> {
        let db = mm.db();

        let task: Task = sqlx::query_as("SELECT * FROM todos WHERE id = $1;")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "todos",
                id,
            })?;

        Ok(task)
    }

    pub async fn list(mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();

        let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM todos ORDER BY id;")
            .fetch_all(db)
            .await?;

        Ok(tasks)
    }
}

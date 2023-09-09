use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get},
    Form, Router,
};
use tracing::info;

use crate::{
    model::{
        tasks::{TaskBmc, TaskForCreate},
        ModelManager,
    },
    templates::{HtmlTemplate, TodoList},
};

pub fn api_routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/hello", get(server_hello))
        .route("/todos", get(list_todo).post(add_todo))
        .route("/todos/:id", delete(delete_todo).patch(switch_done))
        .with_state(mm)
}

async fn server_hello() -> &'static str {
    "Hello!"
}

#[derive(serde::Deserialize)]
struct TodoRequest {
    todo: String,
}

async fn delete_todo(State(mm): State<ModelManager>, Path(id): Path<i64>) -> impl IntoResponse {
    info!("--- Exec delete_todo");

    TaskBmc::delete(&mm, id).await.unwrap();
    let tasks = TaskBmc::list(&mm).await.unwrap();

    let template = TodoList { tasks };

    HtmlTemplate(template)
}

async fn list_todo(State(mm): State<ModelManager>) -> impl IntoResponse {
    info!("--- Exec list_todo");
    let tasks = TaskBmc::list(&mm).await.unwrap();

    let template = TodoList { tasks };

    HtmlTemplate(template)
}

async fn switch_done(State(mm): State<ModelManager>, Path(id): Path<i64>) -> impl IntoResponse {
    info!("--- Exec list_todo");

    TaskBmc::switch_done(&mm, id).await.unwrap();

    let tasks = TaskBmc::list(&mm).await.unwrap();

    let template = TodoList { tasks };

    HtmlTemplate(template)
}

async fn add_todo(
    State(mm): State<ModelManager>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let task_c = TaskForCreate {
        descriptions: todo.todo,
    };

    let _id = TaskBmc::create(&mm, task_c).await;

    let tasks = TaskBmc::list(&mm).await.unwrap();

    let template = TodoList { tasks };

    HtmlTemplate(template)
}

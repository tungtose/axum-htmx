use axum::{extract::State, response::IntoResponse, routing::get, Router};

use crate::{
    model::{tasks::TaskBmc, ModelManager},
    templates::{AboutTemplate, HelloTemplate, HtmlTemplate},
};

pub fn root_routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/about", get(about))
        .with_state(mm)
}

async fn hello(State(mm): State<ModelManager>) -> impl IntoResponse {
    let tasks = TaskBmc::list(&mm).await.unwrap();

    let template = HelloTemplate { tasks };

    HtmlTemplate(template)
}

async fn about() -> impl IntoResponse {
    let template = AboutTemplate {};
    HtmlTemplate(template)
}

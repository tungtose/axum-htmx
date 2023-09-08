mod model;

use anyhow::Context;
use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use model::{
    tasks::{Task, TaskBmc, TaskForCreate},
    ModelManager,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_htmx=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let assets_path = std::env::current_dir().unwrap();

    let mm = ModelManager::new().await?;

    let api_router = Router::new()
        .route("/hello", get(server_hello))
        .route("/todos", post(add_todo));

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/about", get(about))
        .with_state(mm)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

async fn hello(
    State(mm): State<ModelManager>,
) -> impl IntoResponse {
    let tasks = TaskBmc::list(&mm).await.unwrap();
    
    info!("tasks: {:?}", tasks);

    let template = HelloTemplate { tasks };

    HtmlTemplate(template)
}

async fn about() -> impl IntoResponse {
    let template = AboutTemplate {};
    HtmlTemplate(template)
}

async fn server_hello() -> &'static str {
    "Hello!"
}

#[derive(serde::Deserialize)]
struct TodoRequest {
    todo: String,
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

#[derive(Template)]
#[template(path = "todo-list.html")]
struct TodoList {
    tasks: Vec<Task>,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    tasks: Vec<Task>,
}

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

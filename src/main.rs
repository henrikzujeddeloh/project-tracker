use axum::routing::{get, post};
use axum::Router;
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use tower_http::services::ServeDir;

mod db;
mod error;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Server Started");
    // read .env file
    dotenv().ok();

    // connect to mysql database
    let database_url = format!("mysql://{}:{}@{}:{}/{}", &env::var("DB_USERNAME")?, &env::var("DB_PASSWORD")?, &env::var("DB_HOST")?, &env::var("DB_PORT")?, &env::var("DB_NAME")?);
    let pool = MySqlPool::connect(&database_url).await?;
    println!("Connected to database: {}", database_url);

    // set up router
    let app = Router::new()
        .route("/", get(handlers::index_handler))
        .route("/start", post(handlers::start_handler))
        .route("/complete", post(handlers::complete_handler))
        .route("/completed", get(handlers::completed_handler))
        .route("/add", post(handlers::add_handler))
        .route("/delete", post(handlers::delete_handler))
        .route("/up", post(handlers::up_handler))
        .route("/down", post(handlers::down_handler))
        .route("/update_notes", post(handlers::update_notes_handler))
        .route("/project/:id", get(handlers::project_handler))
        .nest_service("/css", ServeDir::new("css"))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(pool);

    // set up listener
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Listening at: {}", &address);
    axum::serve(listener, app).await?;

    Ok(())
}

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
    // read .env file
    dotenv().ok();

    // connect to mysql database
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    println!("Connected to database: {}", &env::var("DATABASE_URL")?);

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

use axum::routing::{get, post};
use axum::Router;
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use tower_http::services::ServeDir;

mod error;
mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Started");
    // read .env file
    dotenv().ok();

    // connect to mysql database
    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    println!("Connected to database: {}", &env::var("DATABASE_URL")?);

    // set up router
    let app = Router::new()
        .route("/", get(handlers::index_handler))
        .nest_service("/css", ServeDir::new("css"))
        .with_state(pool);

    // set up listener
    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Listening at: {}", &address);
    axum::serve(listener, app).await?;

    Ok(())
}

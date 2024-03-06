use askama::Template;
use axum::extract::Form;
use axum::extract::Query;
use axum::extract::State;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;

use crate::error;

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct IndexResponse {}

#[axum_macros::debug_handler]
pub async fn index_handler(
    State(pool): State<MySqlPool>,
) -> Result<IndexResponse, error::AppError> {
    return Ok(IndexResponse {});
}

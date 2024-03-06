use askama::Template;
use axum::extract::Form;
use axum::extract::Query;
use axum::extract::State;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;

use crate::error;
use crate::db;
use crate::models;

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct IndexResponse {
    pub projects: Vec<models::Project>,
}

#[axum_macros::debug_handler]
pub async fn index_handler(
    State(pool): State<MySqlPool>,
) -> Result<IndexResponse, error::AppError> {
    let project_list = db::get_projects(&pool).await?;
    return Ok(IndexResponse { projects: project_list });
}

#[derive(Deserialize, Debug)]
pub struct AddQuery {
    pub name: String,
    pub category: String,
}

#[axum_macros::debug_handler]
pub async fn add_handler(
    State(pool): State<MySqlPool>,
    Query(query): Query<AddQuery>,
) -> Result<IndexResponse, error::AppError> {
    db::add_project(&pool, query.name, query.category).await?;
    let project_list = db::get_projects(&pool).await?;
    return Ok(IndexResponse { projects: project_list });
}


#[derive(Deserialize, Debug)]
pub struct DeleteQuery {
    pub id: u64,
    pub category: String,
}

#[axum_macros::debug_handler]
pub async fn delete_handler(
    State(pool): State<MySqlPool>,
    Query(query): Query<DeleteQuery>,
) -> Result<IndexResponse, error::AppError> {
    db::delete_project(&pool, query.id, query.category).await?;
    let project_list = db::get_projects(&pool).await?;
    return Ok(IndexResponse { projects: project_list });
}

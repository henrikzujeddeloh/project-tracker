use askama::Template;
use axum::extract::Form;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Redirect;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;

use crate::db;
use crate::error;
use crate::models;

const LEFT_CATEGORY: &str = "Personal";
const RIGHT_CATEGORY: &str = "Professional";


// INDEX HANDLER
#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct IndexResponse {
    pub projects: Vec<models::Project>,
    pub left_category: &'static str,
    pub right_category: &'static str,
}

#[axum_macros::debug_handler]
pub async fn index_handler(
    State(pool): State<MySqlPool>,
) -> Result<IndexResponse, error::AppError> {
    let project_list = db::get_projects(&pool).await?;
    return Ok(IndexResponse {
        projects: project_list,
        left_category: LEFT_CATEGORY,
        right_category: RIGHT_CATEGORY,
    });
}

// START HANDLER
#[derive(Deserialize, Debug)]
pub struct StartQuery {
    pub id: u64,
    pub category: String,
    pub position: u64,
}

#[axum_macros::debug_handler]
pub async fn start_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<DeleteQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    db::start_project(&pool, query.id, query.category, query.position).await?;
    Ok(Redirect::to("/"))
}

// COMPLETE HANDLER
#[derive(Deserialize, Debug)]
pub struct CompleteQuery {
    pub id: u64,
}

#[axum_macros::debug_handler]
pub async fn complete_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<CompleteQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    db::complete_project(&pool, query.id).await?;
    Ok(Redirect::to("/completed"))
}

// COMPLETED HANDLER
#[derive(Template, Debug)]
#[template(path = "completed.html")]
pub struct CompletedResponse {
    pub projects: Vec<models::Project>,
}

#[axum_macros::debug_handler]
pub async fn completed_handler(
    State(pool): State<MySqlPool>,
) -> Result<CompletedResponse, error::AppError> {
    let project_list = db::get_projects(&pool).await?;
    return Ok(CompletedResponse {
        projects: project_list,
    });
}

// ADD HANDLER
#[derive(Deserialize, Debug)]
pub struct AddQuery {
    pub name: String,
    pub category: String,
}

#[axum_macros::debug_handler]
pub async fn add_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<AddQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    db::add_project(&pool, query.name, query.category).await?;
    Ok(Redirect::to("/"))
}

// DELETE HANDLER
#[derive(Deserialize, Debug)]
pub struct DeleteQuery {
    pub id: u64,
    pub category: String,
    pub position: u64,
}

#[axum_macros::debug_handler]
pub async fn delete_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<DeleteQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    // TODO: add delete confirmation
    db::delete_project(&pool, query.id, query.category, query.position).await?;
    Ok(Redirect::to("/"))
}

// MOVE HANDLER
#[derive(Deserialize, Debug)]
pub struct MoveQuery {
    pub id: u64,
    pub category: String,
    pub position: u64,
}

#[axum_macros::debug_handler]
pub async fn up_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<MoveQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    db::move_project_up(&pool, query.id, query.category, query.position).await?;
    Ok(Redirect::to("/"))
}

#[axum_macros::debug_handler]
pub async fn down_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<MoveQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    db::move_project_down(&pool, query.id, query.category, query.position).await?;
    Ok(Redirect::to("/"))
}

// UPDATE NOTES HANDLER
#[derive(Deserialize, Debug)]
pub struct UpdateNotesQuery {
    pub id: u64,
    pub notes: String,
}

#[axum_macros::debug_handler]
pub async fn update_notes_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<UpdateNotesQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    db::update_notes(&pool, query.id, query.notes).await?;
    Ok(Redirect::to("/"))
}

// PROJECT HANDLER
#[derive(Template, Debug)]
#[template(path = "project.html")]
pub struct ProjectResponse {
    pub project: models::Project,
}

#[axum_macros::debug_handler]
pub async fn project_handler(
    State(pool): State<MySqlPool>,
    Path(id): Path<u64>,
) -> Result<ProjectResponse, error::AppError> {
    let project = db::get_project(&pool, id).await?;
    Ok(ProjectResponse {
        project: project.expect("Did not find project"),
    })
}

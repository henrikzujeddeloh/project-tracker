use askama::Template;
use axum::extract::Form;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Redirect;
use serde::Deserialize;
use sqlx::mysql::MySqlPool;

use crate::db;
use crate::error;
use crate::models;

const LEFT_CATEGORY: &str = "Personal";
const RIGHT_CATEGORY: &str = "Professional";

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub projects: Vec<models::Project>,
    pub left_category: &'static str,
    pub right_category: &'static str,
}

#[derive(Template, Debug)]
#[template(path = "completed.html")]
pub struct CompletedTemplate {
    pub projects: Vec<models::Project>,
    pub block: Vec<models::Project>,
    pub left_category: &'static str,
    pub right_category: &'static str,
    pub next_block: u64,
    pub more: u8,
}

#[derive(Template, Debug)]
#[template(path = "block.html")]
pub struct BlockTemplate {
    pub block: Vec<models::Project>,
    pub left_category: &'static str,
    pub right_category: &'static str,
    pub next_block: u64,
    pub more: u8,
}


#[derive(Template, Debug)]
#[template(path = "list.html")]
struct ProjectListTemplate<'a> {
    projects: Vec<models::Project>,
    category_name: &'a str,
}

#[derive(Template, Debug)]
#[template(path = "project.html")]
pub struct ProjectTemplate {
    pub project: models::Project,
}

// INDEX HANDLER
#[axum_macros::debug_handler]
pub async fn index_handler(
    State(pool): State<MySqlPool>,
) -> Result<IndexTemplate, error::AppError> {
    let project_list = db::get_projects(&pool).await?;
    return Ok(IndexTemplate {
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
    db::start_project(&pool, query.id, query.category.clone(), query.position).await?;
    let projects = db::get_projects(&pool).await?;
    let context = ProjectListTemplate {
        projects,
        category_name: &query.category,
    };
    let html = context.render()?;
    Ok(Html(html))
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
    Ok(Redirect::to("/completed?block=1"))
}

// COMPLETED HANDLER
#[derive(Deserialize, Debug)]
pub struct BlockQuery {
    pub block: u64,
}

#[axum_macros::debug_handler]
pub async fn completed_handler(
    State(pool): State<MySqlPool>,
    Query(query): Query<BlockQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    // load completed projects html with timeline
    if query.block == 1 {
        let all_completed = db::get_completed_projects(&pool, 0).await?;
        let completed_block = db::get_completed_projects(&pool, 1).await?;
        let mut more_blocks = 1;
        
        if completed_block.len() < 10 {
            more_blocks = 0;
        }
        let context = CompletedTemplate {
            projects: all_completed,
            block: completed_block,
            left_category: LEFT_CATEGORY,
            right_category: RIGHT_CATEGORY,
            next_block: query.block + 1,
            more: more_blocks
        };
        let html = context.render()?;
        Ok(Html(html))

        // load blocks of completed projects
    } else {
        let completed_block = db::get_completed_projects(&pool, query.block).await?;
        let mut more_blocks = 1;
        
        if completed_block.len() < 10 {
            more_blocks = 0;
        }

        let context = BlockTemplate {
            block: completed_block,
            left_category: LEFT_CATEGORY,
            right_category: RIGHT_CATEGORY,
            next_block: query.block + 1,
            more: more_blocks
        };
        let html = context.render()?;
        Ok(Html(html))
    }
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
    db::add_project(&pool, query.name, query.category.clone()).await?;

    let projects = db::get_projects(&pool).await?;
    let context = ProjectListTemplate {
        projects,
        category_name: &query.category,
    };
    let html = context.render()?;
    Ok(Html(html))
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
    db::move_project_up(&pool, query.id, query.category.clone(), query.position).await?;
    let projects = db::get_projects(&pool).await?;
    let context = ProjectListTemplate {
        projects,
        category_name: &query.category,
    };
    let html = context.render()?;
    Ok(Html(html))
}

#[axum_macros::debug_handler]
pub async fn down_handler(
    State(pool): State<MySqlPool>,
    Form(query): Form<MoveQuery>,
) -> Result<impl IntoResponse, error::AppError> {
    db::move_project_down(&pool, query.id, query.category.clone(), query.position).await?;
    let projects = db::get_projects(&pool).await?;
    let context = ProjectListTemplate {
        projects,
        category_name: &query.category,
    };
    let html = context.render()?;
    Ok(Html(html))
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
#[axum_macros::debug_handler]
pub async fn project_handler(
    State(pool): State<MySqlPool>,
    Path(id): Path<u64>,
) -> Result<ProjectTemplate, error::AppError> {
    let project = db::get_project(&pool, id).await?;
    Ok(ProjectTemplate {
        project: project.expect("Did not find project"),
    })
}

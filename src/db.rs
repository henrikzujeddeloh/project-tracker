use sqlx::mysql::MySqlPool;

use crate::models;

pub async fn get_projects(pool: &MySqlPool) -> anyhow::Result<Vec<models::Project>> {
    let projects = sqlx::query_as!(
        models::Project,
        r#"
            SELECT * FROM projects
            ORDER BY position
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(projects)
}

pub async fn get_highest_position_by_category(pool: &MySqlPool, category: &str) -> anyhow::Result<Option<u64>> { 
    let result = sqlx::query!(
        r#"
            SELECT MAX(position) AS max_position
            FROM projects
            WHERE category = ?
        "#,
        category
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|row| row.max_position).expect("no max position found for category"))
}

pub async fn add_project(pool: &MySqlPool, name: String, category: String) -> anyhow::Result<u64> {
    let highest_position = get_highest_position_by_category(pool, &category).await?;
    let position = highest_position.map_or(1, |pos| pos+1);

    let project_id = sqlx::query!(
        r#"
            INSERT INTO projects ( name, category, position )
            VALUES ( ?, ?, ? )
        "#,
        name,
        category,
        position
    )
    .execute(pool)
    .await?
    .last_insert_id();

    println!("Database: added: {} with id {}", name, project_id);

    Ok(project_id)
}

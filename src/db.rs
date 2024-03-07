use sqlx::mysql::MySqlPool;

use crate::models;

// get all projects from database
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

    println!("Database: fetched {} projects", projects.len());
    Ok(projects)
}

pub async fn get_highest_position_by_category(
    pool: &MySqlPool,
    category: &str,
) -> anyhow::Result<Option<u64>> {
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

    Ok(result.map(|row| row.max_position.unwrap_or(0)))
}

pub async fn add_project(pool: &MySqlPool, name: String, category: String) -> anyhow::Result<u64> {
    let highest_position = get_highest_position_by_category(pool, &category).await?;
    let position = highest_position.map_or(1, |pos| pos + 1);

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

    println!(
        "Database: added {} to {} at position {} with id {}",
        name, category, position, project_id
    );

    Ok(project_id)
}

pub async fn delete_project(pool: &MySqlPool, id: u64, category: String) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;

    // find position of the project to be deleted
    let deleted_project_position = sqlx::query!(
        r#"
            SELECT position FROM projects
            WHERE id = ?
        "#,
        id
    )
    .fetch_one(&mut *transaction)
    .await?
    .position;

    // delete the project from the SQL table
    sqlx::query!(
        r#"
            DELETE FROM projects
            WHERE id = ?
        "#,
        id
    )
    .execute(&mut *transaction)
    .await?;

    // adjust the positions of the remaining projects
    sqlx::query!(
        r#"
            UPDATE projects SET position = position - 1 WHERE category = ?
            AND position > ?
        "#,
        category,
        deleted_project_position
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    println!(
        "Database: deleted project from {} at position {} with id {}",
        category, deleted_project_position, id
    );

    Ok(())
}

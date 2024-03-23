use chrono::Local;
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

// get project with id
pub async fn get_project(pool: &MySqlPool, id: u64) -> anyhow::Result<Option<models::Project>> {
    let project = sqlx::query_as!(
        models::Project,
        r#"
            SELECT * FROM projects
            WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    println!("Database: fetched project with id {}", id);

    Ok(project)
}

// start project
pub async fn start_project(
    pool: &MySqlPool,
    id: u64,
    category: String,
    position: u64,
) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;

    let current_date = Local::now();
    // update the status of the started project to 1 and move to position 1
    sqlx::query!(
        r#"
            UPDATE projects 
            SET status = 1, position = 1, start_date = ?
            WHERE id = ?
        "#,
        current_date,
        id
    )
    .execute(&mut *transaction)
    .await?;

    // move all projects before started project position back
    sqlx::query!(
        r#"
            UPDATE projects SET position = position + 1
            WHERE category = ?
            AND position <= ?
            AND id != ?
        "#,
        category,
        position,
        id
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    println!("Database: started project with id {}", id);

    Ok(())
}

// complete project
pub async fn complete_project(pool: &MySqlPool, id: u64) -> anyhow::Result<()> {
    let current_date = Local::now();
    sqlx::query!(
        r#"
            UPDATE projects SET status = 2, completion_date = ?
            WHERE id = ?
        "#,
        current_date,
        id
    )
    .execute(pool)
    .await?;

    print!("Database: completed project with id {}", id);

    Ok(())
}

// get completed projects
pub async fn get_completed_projects(
    pool: &MySqlPool,
    block: u64,
) -> anyhow::Result<Vec<models::Project>> {
    let completed = sqlx::query_as!(
        models::Project,
        r#"
            SELECT * FROM projects
            WHERE status = 2
            ORDER BY completion_date DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    if block > 0 {
        // get block of 10 of completed projects if not first block
        let completed_block = completed
            .into_iter()
            .skip((block - 1) as usize * 10)
            .take(10)
            .collect();
        println!("Database: fetched block {} of completed projects", block);
        Ok(completed_block)
    } else {
        // get all completed projects for timeline
        println!(
            "Database: fetched all ({}) completed projects",
            completed.len()
        );
        Ok(completed)
    }
}

// get highest position in category
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

// add project
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

// delete project
pub async fn delete_project(
    pool: &MySqlPool,
    id: u64,
    category: String,
    position: u64,
) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;

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
            UPDATE projects SET position = position - 1
            WHERE category = ?
            AND position > ?
        "#,
        category,
        position
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    println!(
        "Database: deleted project from {} at position {} with id {}",
        category, position, id
    );

    Ok(())
}

// move project up
pub async fn move_project_up(
    pool: &MySqlPool,
    id: u64,
    category: String,
    position: u64,
) -> anyhow::Result<()> {
    if position == 1 {
        println!("Database: project with id {} already in top position", id);
        return Ok(());
    }

    let mut transaction = pool.begin().await?;

    // move the above project down
    sqlx::query!(
        r#"
            UPDATE projects SET position = position + 1
            WHERE category = ? AND position = ? - 1
        "#,
        category,
        position
    )
    .execute(&mut *transaction)
    .await?;

    // move the current project up
    sqlx::query!(
        r#"
            UPDATE projects SET position = position - 1
            WHERE id = ?
        "#,
        id
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    println!(
        "Database: moved project with id {} up to position {}",
        id,
        position - 1
    );

    Ok(())
}

// move project down
pub async fn move_project_down(
    pool: &MySqlPool,
    id: u64,
    category: String,
    position: u64,
) -> anyhow::Result<()> {
    let highest_position = get_highest_position_by_category(pool, &category)
        .await?
        .map_or(1, |pos| pos);

    if position == highest_position {
        println!(
            "Database: project with id {} already in bottom position ({})",
            id, position
        );
        return Ok(());
    }

    let mut transaction = pool.begin().await?;

    // move the below project up
    sqlx::query!(
        r#"
            UPDATE projects SET position = position - 1
            WHERE category = ? AND position = ? + 1
        "#,
        category,
        position
    )
    .execute(&mut *transaction)
    .await?;

    // move the current project down
    sqlx::query!(
        r#"
            UPDATE projects SET position = position + 1
            WHERE id = ?
        "#,
        id
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    println!(
        "Database: moved project with id {} down to position {}",
        id,
        position + 1
    );

    Ok(())
}

// update notes
pub async fn update_notes(pool: &MySqlPool, id: u64, notes: String) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
            UPDATE projects SET notes = ?
            WHERE id = ?
        "#,
        notes,
        id
    )
    .execute(pool)
    .await?;

    print!("Database: update notes for project with id {}", id);

    Ok(())
}

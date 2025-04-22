use anyhow::Ok;
use sqlx::Result;

use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use tracing::instrument;

// #[tokio::main]
async fn entry() -> anyhow::Result<()> {
    let options = SqliteConnectOptions::new()
        .filename("kesulu.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await.unwrap();

    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS todos
        (
            id          INTEGER PRIMARY KEY NOT NULL,
            description TEXT                NOT NULL,
            done        BOOLEAN             NOT NULL DEFAULT 0
        );
        "#
    )
    .execute(&pool)
    .await?;

    let todo_id = add_todo(&pool, "dd".to_string()).await?;
    list_todos(&pool).await?;

    Ok(())
}

async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
        description
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Done;

#[derive(Debug)]
struct Todo {
    id: i32,
    description: String,
    done: bool,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:123456@localhost/todo")
        .await?;

    let todo_id = add_todo(&pool, "jjj".to_string()).await?;
    list_todos(&pool).await?;

    Ok(())
}

async fn add_todo(pool: &PgPool, description: String) -> anyhow::Result<i32> {
    let rec = sqlx::query!(
        r#"
INSERT INTO todos ( description ,done)
VALUES ( $1,$2)
RETURNING id
        "#,
        description,
        false
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

async fn complete_todo(pool: &PgPool, id: i32) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

async fn list_todos(pool: &PgPool) -> anyhow::Result<()> {
    let recs = sqlx::query_as!(
        Todo,
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;
    dbg!("{}", recs.first().unwrap());

    Ok(())
}

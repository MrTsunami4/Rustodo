#![allow(unused)]

use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::models::Todo;

pub type Db = SqlitePool;

pub async fn init_db() -> Db {
    let pool = SqlitePool::connect("axum.sqlite")
        .await
        .expect("Failed to connect to database");
    pool
}

pub async fn write_todo_db(db: &Db, todo: &Todo) {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    sqlx::query("INSERT INTO todos (id, content) VALUES (?, ?)")
        .bind(todo.id())
        .bind(todo.content())
        .execute(&mut conn)
        .await
        .expect("Failed to write todo to database");
}

pub async fn read_todo_db(db: &Db, id: &Uuid) -> Option<Todo> {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let row = sqlx::query("SELECT id, content FROM todos WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .expect("Failed to read todo from database");

    row.map(|row| Todo::from_db(row.get("id"), row.get("content")))
}

pub async fn read_all_todo_db(db: &Db) -> Vec<Todo> {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let mut todos = Vec::new();

    let rows = sqlx::query("SELECT id, content FROM todos")
        .fetch_all(&mut conn)
        .await
        .expect("Failed to read todos from database");

    for row in rows {
        let id = row.get("id");
        let content = row.get("content");
        todos.push(Todo::from_db(id, content));
    }
    todos
}

pub async fn delete_todo_db(db: &Db, id: &Uuid) -> bool {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let status = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&mut conn)
        .await
        .expect("Failed to delete todo from database");

    status.rows_affected() > 0
}

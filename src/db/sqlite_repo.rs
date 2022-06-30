#![allow(unused)]

use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::{auth::User, models::Todo};

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

    sqlx::query("INSERT INTO todos (id, content, completed) VALUES (?, ?, ?)")
        .bind(todo.id())
        .bind(todo.content())
        .bind(todo.completed())
        .execute(&mut conn)
        .await
        .expect("Failed to write todo to database");
}

pub async fn read_todo_db(db: &Db, id: &Uuid) -> Option<Todo> {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let row = sqlx::query("SELECT id, content, completed FROM todos WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut conn)
        .await
        .expect("Failed to read todo from database");

    row.map(|row| Todo::from_db(row.get("id"), row.get("content"), row.get("completed")))
}

pub async fn read_all_todo_db(db: &Db) -> Vec<Todo> {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let mut todos = Vec::new();

    let rows = sqlx::query("SELECT id, content, completed FROM todos")
        .fetch_all(&mut conn)
        .await
        .expect("Failed to read todos from database");

    for row in rows {
        let id = row.get("id");
        let content = row.get("content");
        let completed = row.get("completed");
        todos.push(Todo::from_db(id, content, completed));
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

pub async fn complete_todo_db(db: &Db, id: &Uuid) -> bool {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let status = sqlx::query("UPDATE todos SET completed = 1 WHERE id = ?")
        .bind(id)
        .execute(&mut conn)
        .await
        .expect("Failed to complete todo in database");

    status.rows_affected() > 0
}

pub async fn user_exist(db: &Db, user: &str) -> Option<User> {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let row = sqlx::query("SELECT user, password FROM users WHERE user = ?")
        .bind(user)
        .fetch_optional(&mut conn)
        .await
        .expect("Failed to read user from database");

    row.map(|row| User::from_db(row.get("user"), row.get("password")))
}

pub async fn create_user(db: &Db, user: &str, password: &str) -> bool {
    let mut conn = db
        .acquire()
        .await
        .expect("Failed to acquire database connection");

    let status = sqlx::query("INSERT INTO users (user, password) VALUES (?, ?)")
        .bind(user)
        .bind(password)
        .execute(&mut conn)
        .await
        .expect("Failed to create user in database");

    status.rows_affected() > 0
}

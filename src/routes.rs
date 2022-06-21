use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use sqlite_repo::Db;
use uuid::Uuid;

use crate::{
    db::sqlite_repo::{self, delete_todo_db, read_all_todo_db, read_todo_db, write_todo_db},
    models::{Todo, TodoRequest},
};

pub async fn create_todo(
    Json(todo): Json<TodoRequest>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    let todo = Todo::new(&todo.content);
    write_todo_db(&db, &todo).await;
    Json(todo)
}

pub async fn read_todo(
    Path(id): Path<Uuid>,
    Extension(db): Extension<Db>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = read_todo_db(&db, &id).await;
    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn read_all_todos(Extension(db): Extension<Db>) -> impl IntoResponse {
    let todos = read_all_todo_db(&db).await;
    Json(todos)
}

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    Extension(db): Extension<Db>,
) -> Result<(), StatusCode> {
    if delete_todo_db(&db, &id).await {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_router() -> Router {
    Router::new()
        .route("/", get(read_all_todos).post(create_todo))
        .route("/:id", get(read_todo).delete(delete_todo))
        .layer(Extension(sqlite_repo::init_db().await))
}

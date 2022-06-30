use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Extension, Json, Router,
};
use jsonwebtoken::{encode, Header};
use sqlite_repo::Db;
use uuid::Uuid;

use crate::{
    auth::User,
    db::sqlite_repo::{
        self, complete_todo_db, create_user, delete_todo_db, read_all_todo_db, read_todo_db,
        user_exist, write_todo_db,
    },
    jwt::{AuthBody, AuthError, AuthPayload, Claims, KEYS},
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

pub async fn complete_todo(
    Path(id): Path<Uuid>,
    Extension(db): Extension<Db>,
) -> Result<(), StatusCode> {
    if complete_todo_db(&db, &id).await {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(claims.to_string())
}

async fn authorize(
    Json(payload): Json<AuthPayload>,
    Extension(db): Extension<Db>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.user.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let user = match user_exist(&db, &payload.user).await {
        Some(user) => user,
        None => return Err(AuthError::WrongCredentials),
    };

    if !user.verify_password(&payload.password) {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        user: user.username.clone(),
        password: user.password,
        exp: 2000000000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

async fn register(
    Json(payload): Json<AuthPayload>,
    Extension(db): Extension<Db>,
) -> impl IntoResponse {
    if user_exist(&db, &payload.user).await.is_some() {
        return Err(StatusCode::CONFLICT);
    }
    let user = User::new(&payload.user, &payload.password);
    let status = create_user(&db, &user.username, &user.password).await;
    if status {
        Ok(())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn get_router() -> Router {
    Router::new()
        .route("/", get(read_all_todos).post(create_todo))
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
        .route("/register", post(register))
        .route("/:id", get(read_todo).delete(delete_todo))
        .route("/:id/complete", put(complete_todo))
        .layer(Extension(sqlite_repo::init_db().await))
}

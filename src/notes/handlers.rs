use super::model::Note;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use axum::extract;

pub async fn index() -> impl IntoResponse {
    (StatusCode::OK, Json(vec![Note {
        id: 1,
        content: "Hello World".to_string(),
    }]))
}

pub async fn new(extract::Json(note): extract::Json<Note>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(note))
}

pub async fn update(extract::Json(note): extract::Json<Note>) -> impl IntoResponse {
    (StatusCode::NO_CONTENT, Json(note))
}

pub async fn delete() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

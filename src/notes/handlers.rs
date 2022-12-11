use super::model::Note;
use axum::{response::IntoResponse, Json};

pub async fn index() -> impl IntoResponse {
    Json(Note {
        id: 1,
        content: "Hello World".to_string(),
    })
}

pub async fn new() -> impl IntoResponse {
    Json(Note {
        id: 1,
        content: "Hello World".to_string(),
    })
}

pub async fn update() -> impl IntoResponse {
    Json(Note {
        id: 1,
        content: "Hello World".to_string(),
    })
}

pub async fn delete() -> impl IntoResponse {
    Json(Note {
        id: 1,
        content: "Hello World".to_string(),
    })
}

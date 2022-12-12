use super::model::Note;
use axum::extract::{self, Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

pub async fn index(State(db): State<SqlitePool>) -> impl IntoResponse {
    let res = sqlx::query_as::<_, Note>("SELECT id, content FROM notes LIMIT 10")
        .fetch_all(&db)
        .await;
    match res {
        Ok(notes) => (StatusCode::OK, Json(notes)),
        Err(e) => {
            println!("error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

#[derive(Deserialize)]
pub struct CreateNoteReq {
    content: String,
}

pub async fn new(State(db): State<SqlitePool>, Json(note): Json<CreateNoteReq>) -> impl IntoResponse {
    let res = sqlx::query("INSERT INTO notes (content) VALUES (?)")
        .bind(note.content)
        .execute(&db)
        .await;
    match res {
        Ok(qr) => (StatusCode::OK, Json(qr.last_insert_rowid())),
        Err(e) => {
            println!("error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(0))
        }
    }
}

pub async fn update(State(db): State<SqlitePool>, Json(note): Json<Note>) -> impl IntoResponse {
    let res = sqlx::query("UPDATE notes SET content=? WHERE id=?")
        .bind(note.content)
        .bind(note.id)
        .execute(&db)
        .await;
    match res {
        Ok(qr) => (StatusCode::OK, Json(())),
        Err(e) => {
            println!("error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
        }
    }
}

#[axum_macros::debug_handler]
pub async fn delete(
    extract::Path(id): extract::Path<i32>,
    State(db): State<SqlitePool>,
) -> impl IntoResponse {
    let res = sqlx::query("DELETE FROM notes WHERE id=?")
        .bind(id)
        .execute(&db)
        .await;
    match res {
        Ok(qr) => (StatusCode::OK, Json(())),
        Err(e) => {
            println!("error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
        }
    }
}

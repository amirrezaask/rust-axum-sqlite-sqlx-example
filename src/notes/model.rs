use serde::{Serialize, Deserialize};
use sqlx::FromRow;

pub type ID = i32;
pub type AttachmentID = u64;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: ID,
    pub content: String,
}


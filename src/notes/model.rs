use serde::{Serialize, Deserialize};

type ID = u64;
type AttachmentID = u64;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: ID,
    pub content: String,
}



use crate::db::Database;

use super::model::ID;
use super::model::Note;

struct NoteDatabase();

impl Database<ID, Note> for NoteDatabase {
    type Error = anyhow::Error;

    fn store(item: Note) -> Result<ID, Self::Error> {
        todo!()
    }

    fn get(id: ID) -> Result<Note, Self::Error> {
        todo!()
    }

    fn update(id: ID, item: Note) -> Result<(), Self::Error> {
        todo!()
    }

    fn delete(id: ID) -> Result<(), Self::Error> {
        todo!()
    }
}

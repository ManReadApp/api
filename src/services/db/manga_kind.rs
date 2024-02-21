use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_extras::SurrealTable;

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("kinds")]
pub struct Kind {
    pub kind: String,
}

pub struct MangaKindDBService {
    conn: Arc<Surreal<Db>>,
}

impl MangaKindDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }
}

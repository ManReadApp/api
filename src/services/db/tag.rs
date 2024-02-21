use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_extras::SurrealTable;

#[derive(SurrealTable, Serialize, Deserialize, Debug, Clone)]
#[db("tags")]
#[sql(["DEFINE EVENT tag_updated ON TABLE tags WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
pub struct Tag {
    pub tag: String,
    pub description: Option<String>,
    pub sex: u64,
}

pub struct TagDBService {
    conn: Arc<Surreal<Db>>,
}

impl TagDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }
}

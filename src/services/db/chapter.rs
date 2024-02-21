use crate::services::db::chapter_version::ChapterVersion;
use crate::services::db::tag::Tag;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::sql::Datetime;
use surrealdb::Surreal;
use surrealdb_extras::{SurrealTable, ThingType};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("chapters")]
#[sql(["DEFINE EVENT chapter_updated ON TABLE chapters WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
pub struct Chapter {
    pub titles: Vec<String>,
    pub chapter: f64,
    pub tags: Vec<ThingType<Tag>>,
    pub sources: Vec<String>,
    pub release_date: Option<Datetime>,
    ///Version to string
    pub versions: HashMap<String, ThingType<ChapterVersion>>,
    #[opt(exclude = true)]
    pub updated: Datetime,
    #[opt(exclude = true)]
    pub created: Datetime,
}

pub struct ChapterDBService {
    conn: Arc<Surreal<Db>>,
}

impl ChapterDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }
}

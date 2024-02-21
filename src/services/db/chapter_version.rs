use crate::services::db::page::Page;
use crate::services::db::version::Version;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::sql::Datetime;
use surrealdb::Surreal;
use surrealdb_extras::{SurrealTable, ThingType};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("chapter_version_connections")]
#[sql(["DEFINE EVENT chapter_version_conn_updated ON TABLE chapter_version_connections WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
pub struct ChapterVersion {
    pub version: ThingType<Version>,
    pub pages: Vec<ThingType<Page>>,
    #[opt(exclude = true)]
    pub updated: Datetime,
    #[opt(exclude = true)]
    pub created: Datetime,
}

pub struct ChapterVersionDBService {
    conn: Arc<Surreal<Db>>,
}

impl ChapterVersionDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }
}

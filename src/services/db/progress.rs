use crate::services::db::chapter_version::ChapterVersion;
use crate::services::db::user::User;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::sql::Datetime;
use surrealdb::Surreal;
use surrealdb_extras::{SurrealTable, ThingType};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("user_progress")]
#[sql(["DEFINE EVENT user_progress_updated ON TABLE user_progress WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
pub struct UserProgress {
    user: ThingType<User>,
    chapter: ThingType<ChapterVersion>,
    #[opt(exclude = true)]
    progress: f64,
    #[opt(exclude = true)]
    updated: Datetime,
}

pub struct ProgressDBService {
    conn: Arc<Surreal<Db>>,
}

impl ProgressDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }
}

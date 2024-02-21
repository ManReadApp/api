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

use crate::services::db::chapter::Chapter;
use crate::services::db::manga::Manga;
use crate::services::db::user::User;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::sql::Datetime;
use surrealdb::Surreal;
use surrealdb_extras::{
    RecordData, SurrealSelect, SurrealTable, SurrealTableInfo, ThingFunc, ThingType,
};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("user_progress")]
#[sql(["DEFINE EVENT user_progress_updated ON TABLE user_progress WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
pub struct UserProgress {
    user: ThingType<User>,
    manga: ThingType<Manga>,
    chapter: ThingType<Chapter>,
    #[opt(exclude = true)]
    progress: f64,
    #[opt(exclude = true)]
    updated: Datetime,
}

#[derive(SurrealSelect, Deserialize)]
pub struct Progress {
    chapter: ThingType<Chapter>,
    progress: f64,
}

pub struct ProgressDBService {
    conn: Arc<Surreal<Db>>,
}

impl ProgressDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }

    pub async fn get_progress(&self, user: &str, manga: ThingFunc) -> Option<(String, f64)> {
        let mut res: Vec<RecordData<Progress>> = User::search(
            &*self.conn,
            Some(format!(
                "WHERE user = users:{} AND manga = {} ORDER BY updated DESC LIMIT 1",
                user,
                manga.to_string()
            )),
        )
        .await
        .ok()?;
        if res.is_empty() {
            None
        } else {
            let v = res.remove(0);
            Some((v.data.chapter.thing.id().to_string(), v.data.progress))
        }
    }
}

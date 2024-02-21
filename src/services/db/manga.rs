use crate::services::db::chapter::Chapter;
use crate::services::db::manga_kind::Kind;
use crate::services::db::tag::Tag;
use crate::services::db::user::User;
use crate::services::db::version::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::sql::Datetime;
use surrealdb::Surreal;
use surrealdb_extras::{SurrealTable, ThingType};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("mangas")]
#[sql(["DEFINE EVENT manga_updated ON TABLE mangas WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
pub struct Manga {
    pub titles: HashMap<String, Vec<String>>,
    pub kind: ThingType<Kind>,
    pub description: Option<String>,
    pub tags: Vec<ThingType<Tag>>,
    pub status: u64,
    pub visibility: u64,
    pub uploader: ThingType<User>,
    pub artists: Vec<ThingType<User>>,
    pub authors: Vec<ThingType<User>>,
    pub covers: Vec<String>,
    pub chapters: Vec<ThingType<Chapter>>,
    pub sources: Vec<String>,
    pub relations: Vec<ThingType<Manga>>,
    pub scraper: Vec<ThingType<Version>>,
    #[opt(exclude = true)]
    pub updated: Datetime,
    #[opt(exclude = true)]
    pub created: Datetime,
}

impl Hash for Manga {
    fn hash<H: Hasher>(&self, _: &mut H) {
        unimplemented!()
    }
}

impl PartialEq<Self> for Manga {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

impl Eq for Manga {}

pub struct MangaDBService {
    conn: Arc<Surreal<Db>>,
}

impl MangaDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }
}

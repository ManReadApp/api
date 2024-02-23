use std::collections::{HashMap};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_extras::{RecordData, SurrealTable, SurrealTableInfo};

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
    temp: HashMap<String, Tag>
}

impl TagDBService {
    async fn get_tag(&mut self, id: &str) -> Option<Tag>{
        if let Some(v) = self.temp.get(id) {
            return Some(v.clone());
        }else {
            let mut hm = HashMap::new();
            let res:Vec<RecordData<Tag>>  = Tag::all(&*self.conn).await.ok()?;
            for item in res {
                hm.insert(item.id.id().to_string(), item.data);
            }
            self.temp = hm;
            self.temp.get(id).copied()
        }
    }

    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn, temp: Default::default() }
    }
}

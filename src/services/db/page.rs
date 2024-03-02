use crate::errors::{ApiError, ApiResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::key::database::pa::Pa;
use surrealdb::sql::Datetime;
use surrealdb::{Connection, Error, Surreal};
use surrealdb_extras::{RecordData, SurrealSelectInfo, SurrealTable, ThingType};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("manga_pages")]
#[sql(["DEFINE EVENT manga_page_updated ON TABLE manga_pages WHEN $event = \"UPDATE\" AND $before.updated == $after.updated THEN (UPDATE $after.id SET updated = time::now() );"])]
pub struct Page {
    pub page: u32,
    pub width: u32,
    pub height: u32,
    pub ext: String,
    pub translation: bool,
    #[opt(exclude = true)]
    pub updated: Datetime,
    #[opt(exclude = true)]
    pub created: Datetime,
}

pub struct PageDBService {
    conn: Arc<Surreal<Db>>,
}

impl PageDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }

    pub async fn get(&self, page: ThingType<Page>) -> ApiResult<Page> {
        let v: RecordData<Page> = page
            .get_part(&*self.conn)
            .await?
            .ok_or(ApiError::db_error())?;
        Ok(v.data)
    }
}

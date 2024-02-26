use crate::services::db::chapter::Chapter;
use crate::services::db::manga_kind::Kind;
use crate::services::db::tag::Tag;
use crate::services::db::user::User;
use crate::services::db::version::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use api_structure::error::{ApiErr, ApiErrorType};
use api_structure::search::{ItemData, ItemOrArray, ItemValue, Order, SearchRequest};
use surrealdb::engine::local::Db;
use surrealdb::sql::Datetime;
use surrealdb::Surreal;
use surrealdb_extras::{RecordData, SurrealSelect, SurrealTable, SurrealTableInfo, ThingType};
use crate::errors::{ApiError, ApiResult};

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

#[derive(SurrealSelect, Deserialize)]
pub struct MangaSearch {
    pub titles: HashMap<String, Vec<String>>,
    pub tags: Vec<ThingType<Tag>>,
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

    pub async fn search(&self, search: SearchRequest, user_id: &str) -> ApiResult<Vec<RecordData<Manga>>> {
        let query = query_builder(search, user_id)?;
        Ok(Manga::search(&*self.conn, Some(query)).await?)
    }
}

enum ItemDataDefined {
    Favorites
}

impl ItemDataDefined {
    fn sql(&self, user_id: &str) -> String {
        match self {
            ItemDataDefined::Favorites => favorites(user_id)
        }
    }
}

impl TryFrom<ItemData> for ItemDataDefined {
    type Error = ApiError;

    fn try_from(value: ItemData) -> Result<Self, Self::Error> {
        if value.name.as_str() == "Favorites" && matches!(value.value, ItemValue::None) {
            return Ok(ItemDataDefined::Favorites);
        }
        Err(ApiErr {
            message: Some("Couldnt find ItemData".to_string()),
            cause: Some(value.name),
            err_type: ApiErrorType::InvalidInput,
        }.into())
    }
}

fn to_sql(item: ItemOrArray, user_id: &str) -> ApiResult<String> {
    Ok(match item {
        ItemOrArray::Item(v) => {
            let item = ItemDataDefined::try_from(v.data)?;
            if v.not {
                format!("NOT {}", item.sql(user_id))
            }else {
                item.sql(user_id)
            }
        },
        ItemOrArray::Array(v) => {
            let mut data = vec![];
            for item in v.items {
                data.push(to_sql(item, user_id)?)
            }
            let join = if v.or {
                "OR"
            } else {
                "AND"
            };
            data.join(&format!(" {} ", join))
        }
    })
}


fn query_builder(r: SearchRequest, user_id: &str) -> ApiResult<String> {
    let asc = if r.desc {
        "DESC"
    } else {
        "ASC"
    };
    //TODO: list_count
    let query = to_sql(r.query, user_id)?;
    let order = format!("ORDER BY {} {}", match r.order {
        Order::Id => "created",
        Order::Alphabetical => "title",
        Order::Updated => "updated",
        Order::LastRead => "read_updated",
        Order::Popularity => "list_count"
    }, asc);
    let limit = format!("LIMIT {} START {}", r.limit, r.page - 1);
    if query.is_empty() {
        Ok(format!("{order} {limit}"))
    }else {
        Ok(format!("WHERE {query} {order} {limit}"))

    }
}

fn last_read() {
    //f32, datetime
    "SELECT progress, updated as read_updated FROM user_progress WHERE user_progress.user = {} AND user_progress.manga = mangas.id ORDER BY user_progress.updated DESC LIMIT 1";
}

fn popularity() {
    // number => list_count
    "count(SELECT id FROM user_progress WHERE user_progress.manga = mangas.id)";
}

fn favorites(user: &str) -> String{
    // true/false => favorite
    format!(r#"count(SELECT id FROM scrape_list WHERE scrape_list.name = "Favorites" AND scrape_list.user = {} scrape_list.mangas CONTAINS mangas.id LIMIT 1) = 1"#, user)
}
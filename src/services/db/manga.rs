use crate::services::db::chapter::Chapter;
use crate::services::db::manga_kind::Kind;
use crate::services::db::tag::Tag;
use crate::services::db::user::User;
use crate::services::db::version::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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

    pub async fn search(&self, search: SearchRequest) -> ApiResult<Vec<RecordData<MangaSearch>>> {
        let query = query_builder(search)?;
        Ok(Manga::search(&*self.conn, Some(query)).await?)
    }
}

enum ItemDataDefined {
    Favorites
}

impl Display for ItemDataDefined {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TryFrom<ItemData> for ItemDataDefined {
    type Error = ApiError;

    fn try_from(value: ItemData) -> Result<Self, Self::Error> {
        if value.name.as_str() == "Favorites" && value.value == ItemValue::None {
            Ok(ItemDataDefined::Favorites)
        }
        Err(ApiErr {
            message: Some("Couldnt find ItemData".to_string()),
            cause: Some(value.name),
            err_type: ApiErrorType::InvalidInput,
        }.into())
    }
}

fn to_sql(item: ItemOrArray) -> ApiResult<String> {
    Ok(match item {
        ItemOrArray::Item(v) => {
            let item = ItemDataDefined::try_from(v.data)?;
            if v.not {
                format!("NOT {}", item)
            }else {
                item.to_string()
            }
        },
        ItemOrArray::Array(v) => {
            let mut data = vec![];
            for item in v.items {
                data.push(to_sql(item)?)
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


fn query_builder(r: SearchRequest) -> ApiResult<String> {
    let asc = if r.desc {
        "DESC"
    } else {
        "ASC"
    };
    //TODO: read_updated,list_count
    let query = to_sql(r.query)?;
    let order = format!("ORDER BY {} {}", match r.order {
        Order::Id => "created",
        Order::Alphabetical => "title",
        Order::Updated => "updated",
        Order::LastRead => "read_updated",
        Order::Popularity => "list_count"
    }, asc);
    let limit = format!("LIMIT {} OFFSET {}", r.limit, r.page - 1);
    Ok(format!("WHERE {query} {order} {limit}"))
}

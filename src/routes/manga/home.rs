use actix_web::post;
use actix_web::web::{Data, Json};
use actix_web_grants::protect;
use api_structure::home::HomeResponse;
use api_structure::search::{Array, Item, ItemData, ItemOrArray, Order, SearchRequest, SearchResponse};
use surrealdb_extras::RecordData;
use crate::errors::{ApiResult};
use crate::services::db::manga::{MangaDBService, MangaSearch};
use crate::services::db::tag::TagDBService;

// #[post("/home")]
// #[protect(
// any("api_structure::auth::role::Role::Admin", "api_structure::auth::role::Role::CoAdmin", "api_structure::auth::role::Role::Moderator", "api_structure::auth::role::Role::Author", "api_structure::auth::role::Role::User"),
// ty = "api_structure::auth::role::Role"
// )]
pub async fn home(manga: Data<MangaDBService>, tags: Data<TagDBService>) -> ApiResult<Json<HomeResponse>> {
    const LIMIT:u32 = 20;
    let trending = SearchRequest {
        order: Order::Popularity,
        desc: true,
        limit: LIMIT,
        page: 1,
        query: ItemOrArray::Array(Array {
            or: false,
            items: vec![],
        }),
    };
    let newest = SearchRequest {
        order: Order::Id,
        desc: true,
        limit: LIMIT,
        page: 1,
        query: ItemOrArray::Array(Array {
            or: false,
            items: vec![],
        }),
    };
    let reading = SearchRequest {
        order: Order::LastRead,
        desc: true,
        limit: LIMIT,
        page: 1,
        query: ItemOrArray::Array(Array {
            or: false,
            items: vec![],
        }),
    };
    let favorites = SearchRequest {
        order: Order::Alphabetical,
        desc: false,
        limit: LIMIT,
        page: 1,
        query: ItemOrArray::Item(Item {
            not: false,
            data: ItemData::enum_("Favorites"),
        }),
    };
    let latest_updates = SearchRequest {
        order: Order::Updated,
        desc: true,
        limit: LIMIT,
        page: 1,
        query: ItemOrArray::Array(Array {
            or: false,
            items: vec![],
        }),
    };
    Ok(Json(HomeResponse {
        trending: format(manga.search(trending).await?, &tags),
        newest: format(manga.search(newest).await?, &tags),
        latest_updates: format(manga.search(latest_updates).await?, &tags),
        favorites: format(manga.search(favorites).await?, &tags),
        reading: format(manga.search(reading).await?, &tags),
    }))
}

fn format(data: Vec<RecordData<MangaSearch>>, tags: &Data<TagDBService>)->Vec<SearchResponse> {
    data.into_iter().map(|v|{
        SearchResponse {
            manga_id: v.id.id().to_string(),
            titles: v.data.titles,
            tags: v.data.tags.iter().map(|v|tags.get_tag(&v.thing.id().to_string())).collect(),
        }
    }).collect()
}
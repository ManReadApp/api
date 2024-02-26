use actix_web::post;
use actix_web::web::{Data, Json, ReqData};
use actix_web_grants::protect;
use api_structure::auth::jwt::Claim;
use api_structure::home::HomeResponse;
use api_structure::search::{Array, Item, ItemData, ItemOrArray, Order, SearchRequest, SearchResponse, Status};
use rand::Rng;
use surrealdb_extras::RecordData;
use crate::errors::{ApiResult};
use crate::services::db::manga::{Manga, MangaDBService, MangaSearch};
use crate::services::db::tag::TagDBService;

#[post("/home")]
#[protect(
any("api_structure::auth::role::Role::Admin", "api_structure::auth::role::Role::CoAdmin", "api_structure::auth::role::Role::Moderator", "api_structure::auth::role::Role::Author", "api_structure::auth::role::Role::User"),
ty = "api_structure::auth::role::Role"
)]
pub async fn home(manga: Data<MangaDBService>, tags: Data<TagDBService>, user: ReqData<Claim>) -> ApiResult<Json<HomeResponse>> {
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
        trending: vec![], //format(manga.search(trending, &user.id).await?, &tags).await,
        newest: format(manga.search(newest, &user.id).await?, &tags).await?,
        latest_updates: format(manga.search(latest_updates, &user.id).await?, &tags).await?,
        favorites: vec![],//format(manga.search(favorites, &user.id).await?, &tags).await,
        reading: vec![],//format(manga.search(reading, &user.id).await?, &tags).await,
    }))
}

pub async fn format(data: Vec<RecordData<Manga>>, tags: &Data<TagDBService>)-> ApiResult<Vec<SearchResponse>> {
    let mut result = vec![];
    for v in data {
        let mut t:Vec<String> = vec![];
        for tag in v.data.tags {
            t.push(tags.get_tag(&tag.thing.id().to_string()).await.unwrap().tag)
        }
        let number = rand::thread_rng().gen_range(0..v.data.covers.len());
        result.push(SearchResponse {
            manga_id: v.id.id().to_string(),
            titles: v.data.titles,
            tags: t,
            status: Status::try_from(v.data.status)?,
            ext: v.data.covers.get(number).unwrap().clone(),
            number: number as u32,
        })
    }
    Ok(result)
}
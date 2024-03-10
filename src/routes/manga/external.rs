//

use crate::errors::ApiResult;
use actix_web::post;
use actix_web::web::{Data, Json};
use actix_web_grants::protect;
use api_structure::scraper::{ExternalSearchRequest, ScrapeSearchResult};
use manread_scraper::SearchService;

#[post("/external/search/sites")]
#[protect(
    any(
        "api_structure::auth::role::Role::Admin",
        "api_structure::auth::role::Role::CoAdmin",
        "api_structure::auth::role::Role::Moderator",
        "api_structure::auth::role::Role::Author",
        "api_structure::auth::role::Role::User"
    ),
    ty = "api_structure::auth::role::Role"
)]
pub async fn available_external_search_sites(
    search_service: Data<SearchService>,
) -> Json<Vec<String>> {
    Json(search_service.sites())
}

#[post("/external/search")]
#[protect(
    any(
        "api_structure::auth::role::Role::Admin",
        "api_structure::auth::role::Role::CoAdmin",
        "api_structure::auth::role::Role::Moderator",
        "api_structure::auth::role::Role::Author",
        "api_structure::auth::role::Role::User"
    ),
    ty = "api_structure::auth::role::Role"
)]
pub async fn search(
    Json(data): Json<ExternalSearchRequest>,
    search_service: Data<SearchService>,
) -> ApiResult<Json<Vec<ScrapeSearchResult>>> {
    Ok(Json(search_service.search(&data.uri, data.data).await?))
}

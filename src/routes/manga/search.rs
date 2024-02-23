use actix_web::web::Json;
use api_structure::home::HomeResponse;
use api_structure::search::SearchRequest;
use crate::errors::ApiResult;

async fn search(Json(request): Json<SearchRequest>) -> ApiResult<Json<HomeResponse>> {
    todo!()
}

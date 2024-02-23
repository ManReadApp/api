use std::fs::File;
use actix_web::web::Json;
use api_structure::home::HomeResponse;
use crate::errors::{ApiError, ApiResult};

async fn home() -> ApiResult<Json<HomeResponse>> {
    File::create()
}
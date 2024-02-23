mod multipart;
mod save;

use crate::env::config::Config;
use crate::errors::{ApiError, ApiResult};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::post;
use actix_web::web::{Data, Json};
use std::path::PathBuf;

#[post("/upload_images")]
pub async fn upload_images(
    data: Multipart,
    config: Data<Config>,
) -> ApiResult<Json<Vec<(String, String)>>> {
    println!("{}", "aaaaa");
    multipart::upload_images(data, config).await.map(Json)
}

#[post("/spinner")]
pub async fn spinner(config: Data<Config>) -> ApiResult<NamedFile> {
    let spinner: PathBuf = config.spinner.clone().into();
    let spinner = config.root_folder.join(spinner);
    let file = NamedFile::open(spinner)?;
    Ok(file)
}

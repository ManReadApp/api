use crate::env::config::Config;
use crate::errors::ApiError;
use actix_files::NamedFile;
use actix_web::web::Data;
use actix_web::{get, web};
use std::path::{Path};

#[get("/{file}")]
async fn frontend_ep(file: web::Path<String>, config: Data<Config>) -> Result<NamedFile, ApiError> {
    frontend_(Some(file.into_inner()), &config.root_folder).await
}

#[get("/")]
async fn frontend_empty_ep(config: Data<Config>) -> Result<NamedFile, ApiError> {
    frontend_(None, &config.root_folder).await
}

async fn frontend_(file: Option<String>, p: &Path) -> Result<NamedFile, ApiError> {
    let file = file
        .map(|mut s| {
            if s.to_lowercase() == "index" {
                s = "index.html".to_string()
            }
            s
        })
        .unwrap_or("index.html".to_string());
    let path = p.join(format!("frontend/{}", file));
    Ok(NamedFile::open(path)?)
}

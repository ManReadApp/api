use crate::env::config::Config;
use crate::errors::ApiResult;
use actix_files::NamedFile;
use actix_web::web::Data;
use actix_web::{get, web, HttpRequest, HttpResponse};
use std::fs::read_to_string;
use std::path::Path;

#[get("/{file}")]
async fn frontend_ep(
    http_request: HttpRequest,
    file: web::Path<String>,
    config: Data<Config>,
) -> ApiResult<HttpResponse> {
    frontend_(
        &http_request,
        Some(file.into_inner()),
        &config.root_folder,
        config.port,
    )
    .await
}

#[get("/")]
async fn frontend_empty_ep(
    http_request: HttpRequest,
    config: Data<Config>,
) -> ApiResult<HttpResponse> {
    frontend_(&http_request, None, &config.root_folder, config.port).await
}

async fn frontend_(
    req: &HttpRequest,
    file: Option<String>,
    p: &Path,
    port: u32,
) -> ApiResult<HttpResponse> {
    let file = file
        .map(|mut s| {
            if s.to_lowercase() == "index" {
                s = "index.html".to_string()
            }
            s
        })
        .unwrap_or("index.html".to_string());
    let path = p.join(format!("frontend/{}", file));
    if file == "index.html" {
        let header = format!(
            "<script>var server_url = 'http://{}:{port}';</script>",
            local_ip_address::local_ip()
                .map(|v| v.to_string())
                .unwrap_or("127.0.0.1".to_string())
        );
        let html = read_to_string(path)?;
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("{header}\n{html}")))
    } else {
        Ok(NamedFile::open(path)?.into_response(&req))
    }
}

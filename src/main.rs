use crate::env::config::Config;
use crate::services::auth_service::validator;
use crate::services::crypto_service::CryptoService;
use crate::services::db::auth_tokens::AuthTokenDBService;
use crate::services::db::chapter::ChapterDBService;
use crate::services::db::chapter_version::ChapterVersionDBService;
use crate::services::db::establish;
use crate::services::db::manga::MangaDBService;
use crate::services::db::manga_kind::MangaKindDBService;
use crate::services::db::manga_list::MangaListDBService;
use crate::services::db::page::PageDBService;
use crate::services::db::progress::ProgressDBService;
use crate::services::db::scrape_account::ScrapeAccountDBService;
use crate::services::db::scrape_list::ScrapeListDBService;
use crate::services::db::tag::TagDBService;
use crate::services::db::user::UserDBService;
use crate::services::db::version::VersionDBService;
use crate::services::uri_service::UriService;
use crate::util::create_folders;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tokio::time::sleep;

mod env;
mod errors;
mod routes;
mod services;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = env::config::get_env().expect("Server configuration failed");
    create_folders(
        &config.root_folder,
        vec![
            "frontend",
            "covers",
            "mangas",
            "ssl",
            "temp",
            "external",
            "users/banner",
            "users/icon",
        ],
    )?;
    #[cfg(feature = "dev")]
    let _ = std::os::unix::fs::symlink(
        std::fs::canonicalize(&config.root_folder).unwrap(),
        PathBuf::from("../scraper/tests"),
    );
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.rust_log.clone()));
    let db = Arc::new(establish(config.root_folder.clone(), true).await.unwrap());
    log_url(&config);
    #[cfg(feature = "https")]
    let ssl_builder = {
        let mut builder =
            openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls())
                .expect("Couldnt initialize SslAcceptor");
        builder
            .set_private_key_file(
                config.root_folder.join("ssl/key.pem"),
                openssl::ssl::SslFiletype::PEM,
            )
            .expect("File does not exist");
        builder
            .set_certificate_chain_file(config.root_folder.join("ssl/cert.pem"))
            .expect("File does not exist");
        builder
    };
    let dbc = db.clone();
    let cfgc = config.clone();
    let hs = HttpServer::new(move || {
        let logger = Logger::default();
        let app = App::new().wrap(logger);
        let external =
            UriService::new(manread_scraper::ExternalSite::init(cfgc.root_folder.clone()).unwrap());
        #[cfg(all(feature = "cors", not(feature = "cors-permissive")))]
        let app = app.wrap(
            actix_cors::Cors::default()
                .allow_any_header()
                .allowed_methods(vec!["GET", "POST"])
                .supports_credentials()
                .max_age(3600),
        );
        #[cfg(all(feature = "cors", feature = "cors-permissive"))]
        let app = app.wrap(actix_cors::Cors::permissive());
        let app = app
            .app_data(Data::from(dbc.clone()))
            .app_data(Data::new(CryptoService {
                secret: cfgc.secret_key.as_bytes().to_vec(),
            }))
            .app_data(Data::new(cfgc.clone()))
            .app_data(Data::new(AuthTokenDBService::new(dbc.clone())))
            .app_data(Data::new(ChapterDBService::new(dbc.clone())))
            .app_data(Data::new(ChapterVersionDBService::new(dbc.clone())))
            .app_data(Data::new(MangaDBService::new(dbc.clone())))
            .app_data(Data::new(MangaKindDBService::new(dbc.clone())))
            .app_data(Data::new(MangaListDBService::new(dbc.clone())))
            .app_data(Data::new(PageDBService::new(dbc.clone())))
            .app_data(Data::new(ProgressDBService::new(dbc.clone())))
            .app_data(Data::new(ScrapeAccountDBService::new(dbc.clone())))
            .app_data(Data::new(ScrapeListDBService::new(dbc.clone())))
            .app_data(Data::new(TagDBService::new(dbc.clone())))
            .app_data(Data::new(UserDBService::new(dbc.clone())))
            .app_data(Data::new(VersionDBService::new(dbc.clone())))
            .app_data(Data::new(external))
            .service(routes::frontend::frontend_ep)
            .service(routes::frontend::frontend_empty_ep)
            .service(
                web::scope("/api")
                    .service(routes::image::upload_images)
                    .service(routes::image::spinner)
                    .service(routes::user::sign_up_route)
                    .service(routes::user::sign_in_route)
                    .service(routes::user::reset_password_route)
                    .service(routes::user::request_reset_password_route)
                    .service(
                        web::scope("")
                            .wrap(HttpAuthentication::bearer(validator))
                            .service(routes::user::refresh_route) //ALL
                            .service(routes::user::activate_route) //NotVerified
                            .service(routes::manga::home_route) //min User
                            .service(routes::manga::search_route) //min User
                            .service(routes::manga::cover_route) //min User
                            .service(routes::manga::info_route) //min User
                            .service(routes::manga::reader_info_route) //min User
                            .service(routes::manga::pages_route) //min User
                            .service(routes::manga::chapter_page_route) //min User
                            .service(routes::manga::translation_route) //min User
                            .service(routes::manga::external_search) //min User
                            .service(routes::manga::available_external_search_sites), //min User
                    ),
            );
        app
    })
    .bind(format!("0.0.0.0:{}", config.port))?;

    #[cfg(feature = "https")]
    let hs = hs.bind_openssl(format!("0.0.0.0:{}", config.https_port), ssl_builder)?;
    let (res, _) = tokio::join!(hs.run(), test(|| { db.clone() }));
    res
}

async fn test(data: impl Fn() -> Arc<Surreal<Db>>) {
    let data = data();
    loop {
        sleep(Duration::from_secs(1)).await
    }
}

fn log_url(config: &Config) {
    #[cfg(feature = "log-ip")]
    if let Ok(ip) = local_ip_address::local_ip() {
        info!("Starting server at http://{}:{}/", &ip, config.port);
    }
    info!("Starting server at http://0.0.0.0:{}/", config.port);
}

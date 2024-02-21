use crate::services::db::auth_tokens::AuthToken;
use crate::services::db::chapter::Chapter;
use crate::services::db::chapter_version::ChapterVersion;
use crate::services::db::manga::Manga;
use crate::services::db::manga_kind::Kind;
use crate::services::db::manga_list::MangaList;
use crate::services::db::page::Page;
use crate::services::db::progress::UserProgress;
use crate::services::db::scrape_account::ScrapeAccount;
use crate::services::db::scrape_list::ScrapeItem;
use crate::services::db::tag::Tag;
use crate::services::db::user::User;
use crate::services::db::version::Version;
use std::path::PathBuf;
use surrealdb::engine::local::{Db, SpeeDb};
use surrealdb::opt::Config;
use surrealdb::Surreal;
use surrealdb_extras::SurrealTableInfo;

pub mod auth_tokens;
pub mod chapter;
pub mod chapter_version;
pub mod manga;
pub mod manga_kind;
pub mod manga_list;
pub mod page;
pub mod progress;
pub mod scrape_account;
pub mod scrape_list;
pub mod tag;
pub mod user;
pub mod version;

pub async fn establish(path: PathBuf) -> surrealdb::Result<Surreal<Db>> {
    let conn = Surreal::new::<SpeeDb>((path.join("db"), Config::default()));
    let register = vec![
        AuthToken::register().expect("Illegal AuthToken structure"),
        Chapter::register().expect("Illegal Chapter structure"),
        ChapterVersion::register().expect("Illegal ChapterVersion structure"),
        Manga::register().expect("Illegal Manga structure"),
        Kind::register().expect("Illegal Kind structure"),
        MangaList::register().expect("Illegal MangaList structure"),
        Page::register().expect("Illegal Page structure"),
        UserProgress::register().expect("Illegal UserProgress structure"),
        ScrapeAccount::register().expect("Illegal ScrapeAccount structure"),
        ScrapeItem::register().expect("Illegal ScrapeItem structure"),
        Tag::register().expect("Illegal Tag structure"),
        User::register().expect("Illegal User structure"),
        Version::register().expect("Illegal Version structure"),
    ];
    surrealdb_extras::use_ns_db(conn, "manread", "manread", register).await
}

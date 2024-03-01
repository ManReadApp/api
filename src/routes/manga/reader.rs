use crate::errors::{ApiError, ApiResult};
use crate::services::db::chapter::ChapterDBService;
use crate::services::db::chapter_version::ChapterVersionDBService;
use crate::services::db::manga::MangaDBService;
use crate::services::db::manga_kind::MangaKindDBService;
use crate::services::db::page::PageDBService;
use crate::services::db::progress::ProgressDBService;
use actix_web::post;
use actix_web::web::{Data, Json, ReqData};
use actix_web_grants::protect;
use api_structure::auth::jwt::Claim;
use api_structure::reader::{
    MangaReaderRequest, MangaReaderResponse, ReaderPage, ReaderPageRequest, ReaderPageResponse,
};
use std::collections::HashMap;

#[post("/pages")]
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
pub async fn get_pages(
    Json(req): Json<ReaderPageRequest>,
    cvs: Data<ChapterVersionDBService>,
    page_s: Data<PageDBService>,
) -> ApiResult<Json<ReaderPageResponse>> {
    let mut pages = HashMap::new();
    for page in cvs.get(&req.chapter_version_id).await? {
        let page = page_s.get(page).await?;
        pages.insert(
            page.page,
            ReaderPage {
                width: page.width,
                height: page.height,
                ext: page.ext,
                translation: page.translation,
            },
        );
    }
    Ok(Json(ReaderPageResponse {
        version_id: req.chapter_version_id,
        hide_top: 0.0,
        hide_bottom: 0.0,
        pages,
    }))
}

#[post("/reader_info")]
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
pub async fn info(
    Json(req): Json<MangaReaderRequest>,
    manga: Data<MangaDBService>,
    chapter_s: Data<ChapterDBService>,
    progress_s: Data<ProgressDBService>,
    user: ReqData<Claim>,
    kind_s: Data<MangaKindDBService>,
) -> ApiResult<Json<MangaReaderResponse>> {
    let manga = manga.get(req.manga_id.as_str()).await?;
    let kind = kind_s
        .get_kind(&manga.data.kind.thing.id().to_string())
        .await
        .ok_or(ApiError::db_error())?;
    let mut chapters = vec![];
    for chapter in manga.data.chapters {
        chapters.push(chapter_s.get_reader(chapter).await?);
    }
    chapters.sort_by(|a, b| a.chapter.partial_cmp(&b.chapter).unwrap());
    let (open_chapter, progress) = match req.chapter_id {
        None => progress_s
            .get_progress(user.id.as_str(), manga.id)
            .await
            .unwrap_or_else(|| {
                (
                    chapters
                        .first()
                        .map(|v| v.chapter_id.clone())
                        .unwrap_or_default(),
                    0.0,
                )
            }),
        Some(v) => (v, 0.0),
    };
    Ok(Json(MangaReaderResponse {
        manga_id: req.manga_id,
        titles: manga.data.titles,
        kind: kind.kind,
        description: manga.data.description,
        chapters,
        favorite: false,
        open_chapter,
        progress,
    }))
}

use crate::error::ApiErr;
use crate::search::Status;
use crate::{ApiErrorType, RequestImpl};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MangaInfoRequest {
    pub manga_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MangaInfoResponse {
    pub manga_id: String,
    pub titles: HashMap<String, Vec<String>>,
    pub kind: String,
    pub description: Option<String>,
    pub tags: Vec<Tag>,
    pub status: Status,
    pub visibility: Visibility,
    pub uploader: String,
    pub my: bool,
    pub artists: Vec<String>,
    pub authors: Vec<String>,
    pub cover: u32,
    pub cover_ext: String,
    pub chapters: Vec<Chapter>,
    pub sources: Vec<ExternalSite>,
    pub relations: Vec<(String, String)>,
    pub scraper: bool,
    pub favorite: bool,
    /// manga_id
    pub progress: Option<String>,
}

impl RequestImpl for MangaInfoRequest {
    const ROUTE: &'static str = "info";
    const AUTH: bool = true;
}

#[derive(Serialize, Deserialize)]
pub struct ExternalSite {
    pub url: String,
    pub icon_uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    pub titles: Vec<String>,
    pub chapter: f64,
    pub tags: Vec<Tag>,
    pub sources: Vec<String>,
    pub release_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub tag: String,
    pub description: Option<String>,
    pub sex: u64,
}

#[derive(Serialize, Deserialize)]
pub enum Visibility {
    /// Everyone
    Visible,
    /// Admins,Coadmins, Mods, and Author
    Hidden,
    /// Admins,Coadmins, Mods
    AdminReview,
}

impl TryFrom<u64> for Visibility {
    type Error = ApiErr;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Visible),
            1 => Ok(Self::Hidden),
            2 => Ok(Self::AdminReview),
            _ => Err(ApiErr {
                message: Some("unknown visibility".to_string()),
                cause: None,
                err_type: ApiErrorType::InternalError,
            }),
        }
    }
}

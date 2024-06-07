use crate::RequestImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MangaCoverRequest {
    pub manga_id: String,
    pub file_ext: String,
}

impl RequestImpl for MangaCoverRequest {
    const ROUTE: &'static str = "cover";
    const AUTH: bool = true;
}

#[derive(Deserialize, Serialize)]
pub struct MangaReaderImageRequest {
    pub manga_id: String,
    pub chapter_id: String,
    pub version_id: String,
    pub page: u32,
    pub file_ext: String,
}

impl RequestImpl for MangaReaderImageRequest {
    const ROUTE: &'static str = "chapter_page";
    const AUTH: bool = true;
}

use crate::search::SearchResponse;
use crate::RequestImpl;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HomeResponse {
    pub trending: Vec<SearchResponse>,
    pub newest: Vec<SearchResponse>,
    pub latest_updates: Vec<SearchResponse>,
    pub favorites: Vec<SearchResponse>,
    pub reading: Vec<SearchResponse>,
    pub random: Vec<SearchResponse>,
}

impl RequestImpl for HomeResponse {
    const ROUTE: &'static str = "home";
    const AUTH: bool = true;
}

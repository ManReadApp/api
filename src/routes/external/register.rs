use std::collections::HashMap;
use std::fs::read_dir;
use std::io::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;
use actix_web::web::Data;
use regex::Regex;
use crate::env::config::Config;

enum Filter {
    StartsWith(String),
    EndsWith(String),
    Contains(String),
    Regex(Regex),
}

pub struct ExternalSite {
    filters: Vec<Filter>,
    path_buf: PathBuf,
    uri: String,
}

impl ExternalSite {
    pub fn init(config: &Config) -> Result<Vec<Self>, String> {
        let mut files = HashMap::new();
        let mut filters = HashMap::new();
        for dir in read_dir(config.root_folder.join("external")).map_err(|e| e.to_string())? {
            let dir = dir.map_err(|e| e.to_string())?;
            let path = dir.path();
            if path.is_file() {
                let name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
                if let Some((name, ext)) = name.split_once(".") {
                    if ext == "filter" {
                        filters.insert(name.to_string(), Filter::new(read_to_string(path).map_err(|e| e.to_string())?)?);
                    } else {
                        files.insert(name.to_string(), path);
                    }
                }
            }
        }
        Ok(filters.into_iter().map(|(site, filter)| ExternalSite {
            filters: filter,
            path_buf: files.get(&site).ok_or("Failed to find file".to_string())?.clone(),
            uri: site,
        }).collect())
    }
}

impl Filter {
    pub fn new(value: String) -> Result<Vec<Self>, String> {
        value.split("\n").map(|v| {
            if let Some(v) = v.strip_prefix("starts_with ") {
                Some(Ok(Filter::StartsWith(v.to_string())))
            } else if let Some(v) = v.strip_prefix("contains ") {
                Some(Ok(Filter::Contains(v.to_string())))
            } else if let Some(v) = v.strip_prefix("regex ") {
                let regex = Regex::from_str(v);
                match regex {
                    Ok(r) => Some(Ok(Filter::Regex(r))),
                    Err(e) => Some(Err(e.to_string()))
                }
            } else if let Some(v) = v.strip_prefix("ends_with ") {
                Some(Ok(Filter::EndsWith(v.to_string())))
            } else {
                None
            }
        }).flatten().collect::<Result<Vec<_>, String>>()
    }

    pub fn check(&self, url: &str) -> bool {
        match self {
            Filter::StartsWith(v) => url.starts_with(v),
            Filter::EndsWith(v) => url.ends_with(v),
            Filter::Contains(v) => url.contains(v),
            Filter::Regex(v) => v.is_match(url)
        }
    }
}
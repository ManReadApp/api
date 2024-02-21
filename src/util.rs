use crate::errors::ApiError;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn create_folders(root: &Path, paths: Vec<&str>) -> std::io::Result<()> {
    for path in paths {
        fs::create_dir_all(root.join(path))?
    }
    Ok(())
}

pub fn create_path(path: &str) -> Result<(), ApiError> {
    fs::create_dir_all(path).map_err(ApiError::write_error)
}
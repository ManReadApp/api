use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FontRequest {
    pub file: String,
}

impl FontRequest {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}

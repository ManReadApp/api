use crate::RequestImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ActivateRequest {
    pub key: String,
}

impl RequestImpl for ActivateRequest {
    const ROUTE: &'static str = "auth/activate";
    const AUTH: bool = true;
}

use crate::RequestImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResetPasswordRequest {
    pub ident: String,
    pub email: bool,
    pub key: String,
    pub password: String,
}

impl RequestImpl for ResetPasswordRequest {
    const ROUTE: &'static str = "auth/reset_password";
    const AUTH: bool = false;
}

#[derive(Deserialize, Serialize)]
pub struct RequestResetPasswordRequest {
    pub ident: String,
    pub email: bool,
}

impl RequestImpl for RequestResetPasswordRequest {
    const ROUTE: &'static str = "auth/request_reset_password";
    const AUTH: bool = false;
}

use crate::RequestImpl;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginWithUsernameAndPassword {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginWithEmailAndPassword {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum LoginRequest {
    Username(LoginWithUsernameAndPassword),
    Email(LoginWithEmailAndPassword),
}

impl RequestImpl for LoginRequest {
    const ROUTE: &'static str = "auth/sign_in";
    const AUTH: bool = false;
}

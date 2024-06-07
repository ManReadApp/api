use crate::RequestImpl;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub birthdate: NaiveDate,
    pub gender: Gender,
    pub icon_temp_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

impl From<usize> for Gender {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Female,
            1 => Self::Male,
            _ => Self::Unknown,
        }
    }
}

impl RequestImpl for NewUserRequest {
    const ROUTE: &'static str = "auth/sign_up";
    const AUTH: bool = false;
}

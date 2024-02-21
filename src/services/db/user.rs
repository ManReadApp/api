use crate::services::db::tag::Tag;
use api_structure::auth::register::Gender;
use api_structure::auth::role::Role;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::sql::Datetime;
use surrealdb::{Error, Surreal};
use surrealdb_extras::{
    Record, RecordData, SurrealSelect, SurrealTable, SurrealTableInfo, ThingFunc,
};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("users")]
pub struct User {
    pub names: Vec<String>,
    pub email: String,
    pub password: String,
    pub role: u32,
    pub bio: Option<String>,
    pub links: Vec<String>,
    pub thumb_ext: Option<String>,
    pub icon_ext: Option<String>,
    pub birthdate: Datetime,
    pub gender: u32,
    #[opt(exclude = true)]
    pub updated: Datetime,
    #[opt(exclude = true)]
    pub created: Datetime,
}

#[derive(SurrealSelect, Deserialize)]
struct Empty {}

pub struct UserDBService {
    pub conn: Arc<Surreal<Db>>,
}

impl UserDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }

    pub async fn new_user(
        &self,
        name: String,
        email: String,
        password: String,
        icon_ext: String,
        bd: NaiveDate,
        gender: Gender,
    ) -> Result<Record, Error> {
        let datetime =
            DateTime::from_naive_utc_and_offset(NaiveDateTime::new(bd, NaiveTime::MIN), Utc);
        let birthdate = Datetime::from(datetime);
        let gender = gender as u32;

        let user = User {
            names: vec![name],
            email,
            password,
            role: Role::NotVerified as u32,
            bio: None,
            links: vec![],
            thumb_ext: None,
            icon_ext: Some(icon_ext),
            birthdate,
            gender,
            updated: Default::default(),
            created: Default::default(),
        };

        user.add_i(&*self.conn).await
    }

    pub async fn email_exists(&self, email: &str) -> bool {
        let result: Vec<RecordData<Empty>> =
            User::search(&*self.conn, Some(format!("email = {}", email)))
                .await
                .unwrap_or_default();
        !result.is_empty()
    }

    pub async fn username_exists(&self, name: &str) -> bool {
        let result: Vec<RecordData<Empty>> =
            User::search(&*self.conn, Some(format!("names CONTAINS {}", name)))
                .await
                .unwrap_or_default();
        !result.is_empty()
    }
}

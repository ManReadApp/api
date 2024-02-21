use crate::services::db::user::User;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::dbs::node::Timestamp;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb_extras::{SurrealTable, ThingType};

#[derive(SurrealTable, Serialize, Deserialize, Debug)]
#[db("auth_tokens")]
pub struct AuthToken {
    user: Option<ThingType<User>>,
    token: String,
    kind: u64,
    active_until_timestamp: Timestamp,
}

pub struct AuthTokenDBService {
    conn: Arc<Surreal<Db>>,
}

impl AuthTokenDBService {
    pub fn new(conn: Arc<Surreal<Db>>) -> Self {
        Self { conn }
    }
}

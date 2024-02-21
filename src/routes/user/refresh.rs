use crate::errors::ApiError;
use crate::services::crypto_service::CryptoService;
use crate::services::db::user::UserDBService;
use actix_web::post;
use actix_web::web::{Data, Json, ReqData};
use api_structure::auth::jwt::{Claim, JWTs};
use surrealdb_extras::SurrealTableInfo;

#[post("/refresh")]
async fn refresh_(
    claim: ReqData<Claim>,
    db: Data<UserDBService>,
    crypto: Data<CryptoService>,
) -> Result<Json<JWTs>, ApiError> {
    let role = db.get_role(claim.id.as_str()).await?;
    Ok(Json(JWTs {
        access_token: crypto.encode_claim(&Claim::new_access(claim.id.clone(), role)?)?,
        refresh_token: crypto.encode_claim(&Claim::new_refresh(claim.id.clone(), role)?)?,
    }))
}

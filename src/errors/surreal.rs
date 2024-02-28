use crate::errors::{ApiError, ApiErrorType};
use api_structure::error::ApiErr;
use surrealdb::Error;

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        ApiErr {
            message: Some("Failed to handle db".to_string()),
            cause: Some(value.to_string()),
            err_type: ApiErrorType::WriteError,
        }
        .into()
    }
}

impl ApiError {
    pub fn db_error() -> Self {
        ApiErr {
            message: Some("couldnt find record".to_string()),
            cause: None,
            err_type: ApiErrorType::InternalError,
        }.into()
    }
}

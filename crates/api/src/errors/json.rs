use crate::errors::ApiError;
use api_structure::error::{ApiErr, ApiErrorType};
use serde_json::Error;

impl From<Error> for ApiError {
    fn from(value: Error) -> Self {
        ApiErr {
            message: Some("failed to parse json".to_string()),
            cause: Some(value.to_string()),
            err_type: ApiErrorType::InternalError,
        }
        .into()
    }
}

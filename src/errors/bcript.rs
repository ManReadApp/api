use crate::errors::{ApiError, ApiErrorType};
use api_structure::error::ApiErr;
use bcrypt::BcryptError;

impl From<BcryptError> for ApiError {
    fn from(value: BcryptError) -> Self {
        ApiErr {
            message: Some("Failed to process jwt".to_string()),
            cause: Some(value.to_string()),
            err_type: ApiErrorType::InvalidInput,
        }
        .into()
    }
}

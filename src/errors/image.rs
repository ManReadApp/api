use crate::errors::{ApiError, ApiErrorType};
use api_structure::error::ApiErr;
use image::ImageError;

impl From<ImageError> for ApiError {
    fn from(value: ImageError) -> Self {
        ApiErr {
            message: Some("Failed to process image".to_string()),
            cause: Some(value.to_string()),
            err_type: ApiErrorType::InvalidInput,
        }
        .into()
    }
}

use crate::errors::{ApiError, ApiErrorType};
use api_structure::error::ApiErr;
use scraper::errror::ScrapeError;

impl From<ScrapeError> for ApiError {
    fn from(error: ScrapeError) -> Self {
        match error {
            ScrapeError::InvalidUrl(v) => ApiErr {
                message: Some("The given URL is invalid".to_string()),
                cause: Some(v),
                err_type: ApiErrorType::InvalidInput,
            },
            ScrapeError::JsSandboxError(v) => ApiErr {
                message: Some("Could not execute the given JS code".to_string()),
                cause: Some(v),
                err_type: ApiErrorType::ReadError,
            },
            ScrapeError::Base64Error(v) => ApiErr {
                message: Some("Could not decode the given base64 string".to_string()),
                cause: Some(v),
                err_type: ApiErrorType::ReadError,
            },
            ScrapeError::KeyDecryptionError(_) => ApiErr {
                message: Some("Could not decrypt the given key".to_string()),
                cause: None,
                err_type: ApiErrorType::ReadError,
            },
            ScrapeError::InputError(v) => ApiErr {
                message: Some("The given input is invalid".to_string()),
                cause: Some(v),
                err_type: ApiErrorType::InvalidInput,
            },
            ScrapeError::FetchError(v) => ApiErr {
                message: Some("Could not fetch the given URL".to_string()),
                cause: Some(v),
                err_type: ApiErrorType::ReadError,
            },
            ScrapeError::ParseError(v) => ApiErr {
                message: Some("Could not parse the given episode".to_string()),
                cause: Some(v),
                err_type: ApiErrorType::ReadError,
            },
            ScrapeError::ReadError(v) => ApiErr {
                message: Some("Could not read the given file".to_string()),
                cause: Some(v),
                err_type: ApiErrorType::ReadError,
            },
        }
        .into()
    }
}

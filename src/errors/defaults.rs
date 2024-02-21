use crate::errors::{ApiError, ApiErrorType};
use api_structure::error::ApiErr;

impl ApiError {
    pub fn unothorized_error(msg: impl ToString, err: impl ToString) -> ApiError {
        ApiErr {
            message: Some(msg.to_string()),
            cause: Some(err.to_string()),
            err_type: ApiErrorType::Unauthorized,
        }
        .into()
    }

    pub fn invalid_token_error(msg: impl ToString, err: impl ToString) -> ApiError {
        ApiErr {
            message: Some(msg.to_string()),
            cause: Some(err.to_string()),
            err_type: ApiErrorType::InvalidInput,
        }
        .into()
    }

    pub fn expired_token_error(msg: impl ToString) -> ApiError {
        ApiErr {
            message: Some(msg.to_string()),
            cause: None,
            err_type: ApiErrorType::InvalidInput,
        }
        .into()
    }

    pub fn multipart_read_error(err: impl ToString) -> ApiError {
        ApiErr {
            message: None,
            cause: Some(err.to_string()),
            err_type: ApiErrorType::ReadError,
        }
        .into()
    }
    pub fn invalid_input(msg: impl ToString) -> ApiError {
        ApiErr {
            message: Some(msg.to_string()),
            cause: None,
            err_type: ApiErrorType::InvalidInput,
        }
        .into()
    }

    pub fn write_error(err: impl ToString) -> ApiError {
        ApiErr {
            message: None,
            cause: Some(err.to_string()),
            err_type: ApiErrorType::WriteError,
        }
        .into()
    }
}

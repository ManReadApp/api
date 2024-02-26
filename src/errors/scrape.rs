use crate::errors::ApiError;
use manread_scraper::error::ScrapeError;

impl From<ScrapeError> for ApiError {
    fn from(error: ScrapeError) -> Self {
        ApiError(error.0)
    }
}

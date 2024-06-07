mod cover;
mod external;
mod home;
mod info;
mod reader;
mod search;

pub use cover::cover_route;
pub use external::available_external_search_sites;
pub use external::search as external_search;
pub use home::home as home_route;
pub use info::info as info_route;
pub use reader::chapter_page_route;
pub use reader::get_pages as pages_route;
pub use reader::info as reader_info_route;
pub use reader::translation as translation_route;
pub use search::search as search_route;
pub use reader::is_valid_translation;

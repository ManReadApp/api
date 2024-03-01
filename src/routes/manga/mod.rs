mod cover;
mod home;
mod info;
mod reader;
mod search;

pub use cover::cover_route;
pub use home::home as home_route;
pub use info::info as info_route;
pub use reader::get_pages as pages_route;
pub use reader::info as reader_info_route;
pub use search::search as search_route;

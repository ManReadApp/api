use crate::get_app_data;
use crate::window_storage::Page;
use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub enum HomePages {
    Home,
    Search,
    You,
    Settings,
}

impl HomePages {
    pub fn all() -> Vec<Self> {
        vec![Self::Home, Self::Search, Self::You, Self::Settings]
    }

    pub fn switch_window(&self) {
        let page = match self {
            HomePages::Home => Page::Home,
            HomePages::Search => Page::Search,
            HomePages::You => Page::You,
            HomePages::Settings => Page::Settings,
        };
        get_app_data().open(page);
    }
}

impl Display for HomePages {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Home => write!(f, "Home"),
            Self::Search => write!(f, "Search"),
            Self::You => write!(f, "You"),
            Self::Settings => write!(f, "Settings"),
        }
    }
}

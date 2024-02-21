use crate::data::user::User;
use crate::window_storage::Page;
use api_structure::auth::jwt::Claim;
use egui::Image;
use reqwest::Client;
use std::collections::HashSet;
use std::mem;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use url::Url;

pub struct SharedData {
    page: Arc<Mutex<Page>>,
    dispose_pages: Arc<Mutex<HashSet<Page>>>,
    pub url: Url,
    user: Arc<Mutex<Option<User>>>,
    pub go_back_page: Arc<Mutex<Option<Page>>>,
    pub client: Client,
    pub spinner: Arc<Mutex<Option<Image<'static>>>>,
}

impl SharedData {
    fn user(&self) -> &Arc<Mutex<Option<User>>> {
        if self.user.lock().unwrap().is_none() {
            self.change(Page::SignIn, Page::all())
        }
        &self.user
    }

    /// gets user data. if none reload page
    pub fn get_user_data(&self) -> Option<Claim> {
        let user = self.user().lock().unwrap();
        user.as_ref().map(|v| v.user_data.clone())
    }

    pub fn logout(&self) {
        *self.user.lock().unwrap() = None;
        self.user();
    }

    /// gets access token. if none reload page
    pub async fn get_access_token(&self) -> Option<String> {
        let mut user = self.user().lock().unwrap().is_some();

        if user {
            let data = {
                let guard = self.user().lock().unwrap();

                if let Some(v) = guard.as_ref().unwrap().get_acces_toke() {
                    Ok(v)
                } else {
                    Err(guard.clone().unwrap())
                }
            };
            return match data {
                Ok(v) => Some(v),
                Err(mut user) => {
                    let v = user.get_new_access_token().await;
                    let res = if let Some(v) = v {
                        *self.user().lock().unwrap() = Some(user);
                        Some(v)
                    } else {
                        *self.user.lock().unwrap() = None;
                        self.user();
                        None
                    };
                    return res;
                }
            };
        }
        None
    }

    pub fn set_user_data(&self, user: User) {
        *self.user.lock().unwrap() = Some(user);
    }

    pub fn page(&self) -> Page {
        *self.page.lock().unwrap()
    }
    pub fn new() -> Self {
        Self {
            page: Arc::new(Mutex::new(Page::LoadingInitRefresh)),
            dispose_pages: Default::default(),
            url: Url::from_str("http://127.0.0.1:8082/api/").unwrap(),
            user: Default::default(),
            go_back_page: Arc::new(Mutex::new(None)),
            client: Default::default(),
            spinner: Default::default(),
        }
    }

    pub fn open(&self, page: Page) {
        *self.page.lock().unwrap() = page;
    }

    pub fn clean(&self, pages: Vec<Page>) {
        let mut dis = self.dispose_pages.lock().unwrap();
        for page in pages {
            dis.insert(page);
        }
    }

    pub fn change(&self, page: Page, pages: Vec<Page>) {
        self.open(page);
        self.clean(pages);
    }

    pub fn change_window(&self) -> Option<HashSet<Page>> {
        let mut dis = self.dispose_pages.lock().unwrap();
        if dis.len() > 0 {
            let mut hs = HashSet::new();
            mem::swap(&mut *dis, &mut hs);
            Some(hs)
        } else {
            None
        }
    }
}

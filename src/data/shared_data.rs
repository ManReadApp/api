use std::collections::HashSet;
use std::mem;
use std::sync::{Arc, Mutex};
use crate::window_storage::Page;

pub struct SharedData {
    page: Arc<Mutex<Page>>,
    dispose_pages: Arc<Mutex<HashSet<Page>>>
}

impl SharedData {
    pub fn page(&self) -> Page {
        *self.page.lock().unwrap()
    }
    pub fn new() -> Self{
        Self {
            page: Arc::new(Mutex::new(Page::Loading)),
            dispose_pages: Default::default(),
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
        }else {
            None
        }
    }
}
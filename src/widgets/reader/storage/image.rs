use crate::get_app_data;
use crate::widgets::reader::overlay::ReaderTranslationArea;
use api_structure::now_timestamp;
use egui::{include_image, Image};
use ethread::ThreadHandler;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub struct ImageStorage {
    hashmap: HashMap<String, ImageStore>,
    pub(crate) loading: Arc<(Image<'static>, Vec<ReaderTranslationArea>)>,
    pub(crate) error: Arc<(Image<'static>, Vec<ReaderTranslationArea>)>,
}

impl Default for ImageStorage {
    fn default() -> Self {
        Self {
            hashmap: Default::default(),
            loading: Arc::new((
                get_app_data().spinner.lock().unwrap().clone().unwrap(),
                vec![],
            )),
            error: Arc::new((
                Image::from(include_image!("../../../assets/error.gif")),
                vec![],
            )),
        }
    }
}

impl ImageStorage {
    pub(crate) fn get(&mut self, s: &str) -> Option<&ImageStore> {
        if let Some(v) = self.hashmap.get_mut(s) {
            v.last_checked = now_timestamp().unwrap();
            Some(v)
        } else {
            None
        }
    }

    pub(crate) fn insert(
        &mut self,
        key: String,
        image: ThreadHandler<Option<Arc<(Image, Vec<ReaderTranslationArea>)>>>,
    ) {
        self.hashmap.insert(
            key,
            ImageStore {
                last_checked: Default::default(),
                req: image,
            },
        );
    }
}

pub struct ImageStore {
    last_checked: Duration,
    pub(crate) req: ThreadHandler<Option<Arc<(Image<'static>, Vec<ReaderTranslationArea>)>>>,
}

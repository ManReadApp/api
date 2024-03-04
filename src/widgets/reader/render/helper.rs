use crate::widgets::reader::overlay::ReaderTranslationArea;
use crate::widgets::reader::storage::{ImageStorage, State};
use api_structure::reader::{Action, ReaderPage};
use egui::Image;
use std::sync::Arc;

pub fn get_img_(
    imgs: &mut ImageStorage,
    ch: State,
    page: u32,
) -> Option<(
    ReaderPage,
    Arc<(Image<'static>, Vec<ReaderTranslationArea>)>,
)> {
    match ch {
        State::ReaderPageResponse(v_) => match v_.get_page(page) {
            Action::Page(r) => {
                let img = get_img(&r.page_id, imgs);
                Some((r.clone(), img))
            }
            _ => None,
        },
        State::ChapterLoading => Some((ReaderPage::new(100, 100), imgs.loading.clone())),
        State::ChapterError => Some((ReaderPage::new(100, 100), imgs.error.clone())),
        State::NoChapter => Some((ReaderPage::new(100, 100), imgs.error.clone())),
    }
}

pub fn get_img<'a>(
    page_id: &'a str,
    imgs: &'a mut ImageStorage,
) -> Arc<(Image<'static>, Vec<ReaderTranslationArea>)> {
    if let Some(img) = imgs.get(page_id) {
        if let Some(v) = img.req.task.ready() {
            match v {
                None => imgs.error.clone(),
                Some(v) => return v.clone(),
            }
        } else {
            imgs.loading.clone()
        }
    } else {
        unreachable!()
    }
}

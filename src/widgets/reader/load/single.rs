use crate::widgets::reader::load::load_image;
use crate::widgets::reader::progress::Progress;
use crate::widgets::reader::storage::ImageStorage;
use api_structure::reader::{MangaReaderResponse, ReaderPageResponse};
use std::sync::Arc;

pub fn single(
    v: &Arc<MangaReaderResponse>,
    hierachy: &[String],
    chapter: &str,
    area: f32,
    progress: &Progress,
    reader_page: &Arc<ReaderPageResponse>,
    i_: u32,
    imgs_: &mut ImageStorage,
) {
    let area = area as u32 * i_;
    for i in 0..=area + i_ {
        if load_image(&v, hierachy, chapter, progress, imgs_, &reader_page, i).0 {
            break;
        }
        if i != 0 && i <= area {
            if load_image(&v, hierachy, chapter, progress, imgs_, &reader_page, i).0 {
                break;
            }
        }
    }
}

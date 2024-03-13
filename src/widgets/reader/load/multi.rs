use crate::widgets::reader::load::load_image;
use crate::widgets::reader::progress::Progress;
use crate::widgets::reader::storage::ImageStorage;
use api_structure::reader::{Action, MangaReaderResponse, ReaderPageResponse};
use egui::Context;
use std::sync::Arc;

pub fn multi(
    v: &Arc<MangaReaderResponse>,
    hierachy: &[String],
    chapter: &str,
    area: f32,
    progress: &Progress,
    reader_page: &Arc<ReaderPageResponse>,
    x: f32,
    y: f32,
    height: bool,
    imgs_: &mut ImageStorage,
    ctx: &Context,
) {
    let area = y * area;
    let mut processed = 0.;
    let mut count = 0;
    while processed < area + y {
        let (br, img) = load_image(
            &v,
            hierachy,
            chapter,
            progress,
            imgs_,
            &reader_page,
            count,
            ctx,
        );
        match img {
            Action::Page(v) => {
                let a = if height { v.height(x) } else { v.width(x) };
                processed += a;
            }
            _ => {}
        }
        if br {
            break;
        }
        count += 1;
    }
}

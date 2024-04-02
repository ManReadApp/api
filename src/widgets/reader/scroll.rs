use crate::widgets::reader::progress::Progress;
use crate::widgets::reader::settings::ReadingMode;
use crate::widgets::reader::storage::{get_page_resp, PageData, State};
use api_structure::reader::{Action, MangaReaderResponse};
use eframe::emath::Vec2;
use egui::Ui;
use std::sync::Arc;
fn get_scroll_delta(ui: &mut Ui) -> Vec2 {
    ui.input(|i| i.smooth_scroll_delta)
}
pub fn set_progress(
    ui: &mut Ui,
    rm: &ReadingMode,
    progress: &mut Progress,
    mrr: Arc<MangaReaderResponse>,
    hierachy: &[String],
    page_data: &mut PageData,
    area: Vec2,
) {
    match rm {
        ReadingMode::Strip => {
            let scroll_delta = get_scroll_delta(ui);
            let mut ch = get_page_resp(
                mrr.clone(),
                hierachy,
                page_data,
                &progress.chapter,
                ui.ctx(),
            );
            if let State::ReaderPageResponse(v) = ch {
                match v.get_page(progress.image as i32) {
                    Action::Page(page) => {
                        let max = page.height(area.x);
                        let processed = progress.pixels - scroll_delta.y;
                        if processed > max {
                            match v.get_page((progress.image + 1) as i32) {
                                Action::Prev => unreachable!(),
                                Action::Page(_) => {
                                    progress.image += 1;
                                    progress.pixels = processed - max;
                                }
                                Action::Next => {
                                    //TODO:
                                }
                            }
                        } else if processed < 0.0 {
                            match v.get_page(progress.image as i32 - 1) {
                                Action::Prev => {
                                    //TODO:
                                }
                                Action::Page(v) => {
                                    progress.image -= 1;
                                    progress.pixels = v.height(area.x);
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            progress.pixels = processed;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        ReadingMode::Row(_) => {}
        ReadingMode::Single => {}
        ReadingMode::Double(_) => {}
    }
}

use crate::fetcher::{Complete, Fetcher};
use crate::get_app_data;
use crate::widgets::reader::load::load_images;
use crate::widgets::reader::progress::Progress;
use crate::widgets::reader::render::display_images;
use crate::widgets::reader::settings::{ReadingMode, Settings, ViewArea};
use crate::widgets::reader::storage::{get_version, Storage};
use api_structure::reader::MangaReaderRequest;
use api_structure::RequestImpl;
use eframe::{App, Frame};
use egui::{vec2, Context};

pub struct MangaReaderPage {
    storage: Storage,
    settings: Settings,
    progress: Option<Progress>,
    init: bool,
}

impl MangaReaderPage {
    pub(crate) fn new(manga_id: String, chapter_id: Option<String>) -> Self {
        let mut manga = Fetcher::new(MangaReaderRequest::request(&get_app_data().url).unwrap());
        manga.set_body(MangaReaderRequest {
            manga_id,
            chapter_id,
        });
        manga.send();
        Self {
            storage: Storage {
                manga,
                page_data: Default::default(),
                loaded_pages: Default::default(),
            },
            settings: Settings {
                version_hierachy: vec![],
                reading_mode: ReadingMode::Strip,
                prefetch: 3.0,
                view_area: ViewArea {
                    margin_top: 0.0,
                    margin_right: 0.0,
                    margin_bottom: 0.0,
                    margin_left: 0.0,
                },
            },
            progress: None,
            init: false,
        }
    }
}

impl App for MangaReaderPage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if !self.init {
            self.init = true;
            self.storage.manga.set_ctx(ctx.clone());
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(v) = self.storage.manga.result().cloned() {
                match v {
                    Complete::Json(v) => {
                        if let Some(p) = &self.progress {
                            let size = self.settings.view_area.get_size(ctx);
                            load_images(
                                v.clone(),
                                &self.settings.version_hierachy,
                                &mut self.storage.page_data,
                                &p.chapter,
                                ctx,
                                &self.settings.reading_mode,
                                self.settings.prefetch,
                                p,
                                size,
                                &mut self.storage.loaded_pages,
                            );
                            display_images(
                                v.clone(),
                                &self.settings.version_hierachy,
                                &mut self.storage.page_data,
                                p,
                                ui,
                                &self.settings.reading_mode,
                                &mut self.storage.loaded_pages,
                                size,
                                vec2(
                                    self.settings.view_area.margin_left,
                                    self.settings.view_area.margin_top,
                                ),
                            )
                        } else {
                            if v.no_chapters() {
                                //TODO: no chapters
                            } else {
                                let ver = get_version(
                                    v.get_chapter(&v.open_chapter).unwrap(),
                                    &self.settings.version_hierachy,
                                );
                                self.progress = Some(Progress {
                                    chapter: v.open_chapter.to_string(),
                                    image: 1,
                                    pixels: 50.0,
                                });
                                //TODO: create progress
                            }
                        }
                    }
                    _ => v.display_error(ui),
                }
            }
        });
    }
}

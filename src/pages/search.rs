use crate::fetcher::{Complete, Fetcher};
use crate::get_app_data;
use crate::widgets::image_overlay::ImageOverlay;
use crate::window_storage::Page;
use api_structure::search::{SearchRequest, SearchResponse};
use api_structure::RequestImpl;
use chrono::Duration;
use eframe::emath::vec2;
use eframe::{App, Frame};
use egui::scroll_area::ScrollBarVisibility;
use egui::{Context, Grid, Image, Label, ScrollArea, Sense, Spinner};
use ethread::ThreadHandler;
use log::error;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::mem;

pub struct SearchPage {
    internal: SearchData<SearchResponse>,
}

pub struct SearchData<D: DeserializeOwned> {
    searched: Vec<D>,
    fetcher: Fetcher<Vec<D>>,
    init: bool,
    end: bool,
}

impl SearchPage {
    pub fn new() -> Self {
        let mut fetcher: Fetcher<Vec<SearchResponse>> =
            Fetcher::new(SearchRequest::request(&get_app_data().url).unwrap());
        fetcher.set_body(&*get_app_data().search.lock().unwrap());
        fetcher.send();
        Self {
            internal: SearchData {
                searched: vec![],
                fetcher,
                init: false,
                end: false,
            },
        }
    }

    fn move_data(&mut self, ctx: &Context) {
        if !self.internal.init {
            self.internal.init = true;
            self.internal.fetcher.set_ctx(ctx.clone())
        }
        if self.internal.fetcher.result().is_some() {
            let mut new = Fetcher::new_ctx(
                SearchRequest::request(&get_app_data().url).unwrap(),
                ctx.clone(),
            );
            mem::swap(&mut new, &mut self.internal.fetcher);
            let result = new.take_result().unwrap();
            match result {
                Complete::ApiError(e) => error!("{:?}", e),
                Complete::Error(e) => error!("{:?}", e),
                Complete::Bytes(_) => unreachable!(),
                Complete::Json(mut v) => {
                    if v.is_empty() {
                        self.internal.end = true
                    } else {
                        self.internal.searched.append(&mut v);
                        get_app_data().search.lock().unwrap().page += 1;
                    }
                }
            }
            self.internal
                .fetcher
                .set_body(&*get_app_data().search.lock().unwrap());
        }
    }
}

impl App for SearchPage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        self.move_data(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Search");
            let itemsxrow = 5f32;
            let size = (ui.available_width() + ui.spacing().item_spacing.x) / itemsxrow - 10.;
            ScrollArea::vertical()
                .drag_to_scroll(true)
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    let app = get_app_data();
                    Grid::new("search_grid")
                        .num_columns(
                            (self.internal.searched.len() as f32 / itemsxrow).ceil() as usize
                        )
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            for (index, item) in self.internal.searched.iter().enumerate() {
                                if index != 0 && index % itemsxrow as usize == 0 {
                                    ui.end_row();
                                }
                                ui.vertical(|ui| {
                                    let image = {
                                        app.covers.lock().unwrap().get(
                                            &item.manga_id,
                                            &item.status,
                                            &item.ext,
                                            item.number,
                                        )
                                    };
                                    if let Some(img) = image {
                                        let img = img.fit_to_exact_size(vec2(size, size * 1.5));
                                        if ui.add(img).clicked() {
                                            get_app_data()
                                                .open(Page::MangaInfo(item.manga_id.clone()))
                                        }
                                    } else {
                                        let (rect, _) = ui.allocate_exact_size(
                                            vec2(size, size * 1.5),
                                            Sense::hover(),
                                        );
                                        let spinner = Spinner::new();

                                        ui.put(rect, spinner);
                                    }

                                    let v = ui.allocate_exact_size(vec2(size, 40.), Sense::hover());
                                    let label = Label::new(get_app_data().get_title(&item.titles))
                                        .sense(Sense::click());
                                    if ui.put(v.0, label).clicked() {
                                        get_app_data().open(Page::MangaInfo(item.manga_id.clone()))
                                    }
                                });
                            }
                        });
                });
        });
    }
}

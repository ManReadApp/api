use crate::fetcher::{Complete, Fetcher};
use crate::get_app_data;
use crate::widgets::image_overlay::ImageOverlay;
use crate::window_storage::Page;
use api_structure::search::{DisplaySearch, Field, ItemKind, SearchRequest, SearchResponse, Status};
use api_structure::RequestImpl;
use chrono::Duration;
use eframe::emath::vec2;
use eframe::{App, Frame};
use egui::scroll_area::ScrollBarVisibility;
use egui::{Color32, Context, Grid, Image, Label, OpenUrl, ScrollArea, Sense, Spinner, TextEdit, Ui, Vec2};
use ethread::ThreadHandler;
use log::{error, info};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::mem;
use crate::util::parser::search_parser;

pub struct SearchPage {
    internal: SearchData<SearchResponse>,
}

pub struct SearchData<D: DisplaySearch> {
    searched: Vec<D>,
    fetcher: Fetcher<Vec<D>>,
    search: String,
    init: bool,
    end: bool,
    require_new: bool,
}

impl<D: DisplaySearch> SearchData<D> {
    pub fn set_load(&mut self, need: impl FnOnce() -> bool) {
        if self.end {
            self.require_new = false
        } else {
            self.require_new = need()
        }
    }
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
                search: "".to_string(),
                init: false,
                end: false,
                require_new: false,
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

        if self.internal.require_new && !self.internal.fetcher.loading() {
            self.internal.fetcher.send()
        }
    }
}
fn display_grid<T: DisplaySearch>(ui: &mut Ui, data: &mut SearchData<T>) {
    let height = ui.available_height();
    let itemsxrow = (ui.available_width() / 200.).floor();
    let size = (ui.available_width() + ui.spacing().item_spacing.x) / itemsxrow - 10.;
    let v = ScrollArea::vertical()
        .drag_to_scroll(true)
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            let app = get_app_data();
            Grid::new("search_grid")
                .num_columns((data.searched.len() as f32 / itemsxrow).ceil() as usize)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for (index, item) in data.searched.iter().enumerate() {
                        if index != 0 && index % itemsxrow as usize == 0 {
                            ui.end_row();
                        }
                        ui.vertical(|ui| {
                            let image = {
                                if item.internal() {
                                    app.covers.lock().unwrap().get(
                                        &item.id_url(),
                                        &item.status(),
                                        &item.ext(),
                                        item.image_number(),
                                    )
                                } else {
                                    app.covers.lock().unwrap().get_url(&item.id_url())
                                }
                            };
                            if let Some(img) = image {
                                let img = img.fit_to_exact_size(vec2(size, size * 1.5));
                                if ui.add(img).clicked() {
                                    if item.internal() {
                                        get_app_data().open(Page::Reader {
                                            manga_id: item.id_url().clone(),
                                            chapter_id: None,
                                        })
                                    } else {
                                        todo!("display infos in app")
                                    }
                                }
                            } else {
                                let (rect, _) =
                                    ui.allocate_exact_size(vec2(size, size * 1.5), Sense::hover());
                                let spinner = Spinner::new();

                                ui.put(rect, spinner);
                            }

                            let v = ui.allocate_exact_size(vec2(size, 40.), Sense::hover());
                            let label = Label::new(get_app_data().get_title(&item.titles()))
                                .sense(Sense::click());
                            if ui.put(v.0, label).clicked() {
                                if item.internal() {
                                    get_app_data().open(Page::MangaInfo(item.id_url().clone()))
                                } else {
                                    let modifiers = ui.ctx().input(|i| i.modifiers);
                                    ui.ctx().open_url(OpenUrl {
                                        url: item.id_url().to_string(),
                                        new_tab: modifiers.any(),
                                    });
                                }
                            }
                        });
                    }
                });
        });
    data.set_load(|| (v.content_size.y - v.state.offset.y) < (height * 3.));
}

impl App for SearchPage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        self.move_data(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let (parsed, errors) = search_parser(&self.internal.search, false, &vec![Field::new("title".to_string(), vec![String::new(), "t".to_string()], ItemKind::String)]);
            let color = if !errors.is_empty() {
                Some(Color32::from_rgb(255,64, 64))
            }else {
                None
            };
            let mut search_field = TextEdit::singleline(&mut self.internal.search);
            if let Some(color) = color {
                search_field = search_field.text_color(color)
            }
            let resp = ui.add(search_field.margin(vec2(10.,10.)).hint_text("Advanced Search").desired_width(ui.available_width()));
            if !errors.is_empty() {
                resp.on_hover_text(errors.join("\n"));
            }
            info!("{:?}", parsed);
            ui.add_space(10.);
            display_grid(ui, &mut self.internal);
        });
    }
}

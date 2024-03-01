use crate::fetcher::{Complete, Fetcher};
use crate::get_app_data;
use crate::widgets::home_page_swithcer::HomePages;
use crate::widgets::image_overlay::ImageOverlay;
use crate::window_storage::Page;
use api_structure::auth::role::Role;
use api_structure::home::HomeResponse;
use api_structure::image::MangaCoverRequest;
use api_structure::search::{
    Array, Item, ItemData, ItemOrArray, Order, SearchRequest, SearchResponse, Status,
};
use api_structure::RequestImpl;
use eframe::{App, Frame};
use egui::scroll_area::ScrollBarVisibility;
use egui::{
    vec2, Align, Button, Context, Grid, Image, Label, Layout, ScrollArea, Sense, Spinner, Ui,
};
use ethread::ThreadHandler;
use futures_util::{stream, StreamExt};
use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;

pub struct HomePage {
    data: Fetcher<Arc<HomeResponse>>,
    img: Option<ThreadHandler<Arc<HashMap<String, ImageOverlay>>>>,
    init: bool,
}

impl Default for HomePage {
    fn default() -> Self {
        let mut req = Fetcher::new(HomeResponse::request(&get_app_data().url).unwrap());
        req.send();
        Self {
            data: req,
            img: None,
            init: false,
        }
    }
}

impl App for HomePage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if !self.init {
            self.init = true;
            self.data.set_ctx(ctx.clone())
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(data) = self.data.result().cloned() {
                if let Complete::Json(v) = data {
                    if let Some(imgs) = &self.img {
                        if let Some(imgs) = imgs.task.ready().cloned() {
                            self.show(ui, &v, &imgs)
                        } else {
                            ui.add(get_app_data().spinner.lock().unwrap().clone().unwrap());
                        }
                    } else {
                        let mut items = v
                            .newest
                            .iter()
                            .map(|v| (v.manga_id.clone(), (v.status, v.ext.clone(), v.number)))
                            .collect::<Vec<_>>();
                        items.append(
                            &mut v
                                .latest_updates
                                .iter()
                                .map(|v| (v.manga_id.clone(), (v.status, v.ext.clone(), v.number)))
                                .collect(),
                        );
                        items.append(
                            &mut v
                                .favorites
                                .iter()
                                .map(|v| (v.manga_id.clone(), (v.status, v.ext.clone(), v.number)))
                                .collect(),
                        );
                        items.append(
                            &mut v
                                .trending
                                .iter()
                                .map(|v| (v.manga_id.clone(), (v.status, v.ext.clone(), v.number)))
                                .collect(),
                        );
                        items.append(
                            &mut v
                                .reading
                                .iter()
                                .map(|v| (v.manga_id.clone(), (v.status, v.ext.clone(), v.number)))
                                .collect(),
                        );
                        let ids = items.into_iter().collect::<HashMap<_, _>>();
                        let req = async {
                            let app = get_app_data();
                            let reqs = ids.into_iter().map(
                                |(manga_id, (status, ext, number))| async move {
                                    let token =
                                        format!("Bearer {}", app.get_access_token().await.unwrap());
                                    let bytes = app
                                        .client
                                        .post(app.url.join("cover").unwrap())
                                        .header(AUTHORIZATION, token)
                                        .json(&MangaCoverRequest {
                                            manga_id: manga_id.clone(),
                                            file_ext: ext,
                                        })
                                        .send()
                                        .await
                                        .ok()?
                                        .bytes()
                                        .await
                                        .ok()?;
                                    let img = Image::from_bytes(
                                        format!("cover://{}", manga_id),
                                        bytes.to_vec(),
                                    )
                                    .sense(Sense::click())
                                    .fit_to_exact_size(vec2(200., 300.));
                                    let overlay = match status {
                                        Status::Dropped => ImageOverlay::dropped(img),
                                        Status::Hiatus => ImageOverlay::hiatus(img),
                                        Status::Ongoing => ImageOverlay::ongoing(img),
                                        Status::Completed => ImageOverlay::completed(img),
                                        Status::Upcoming => ImageOverlay::upcoming(img),
                                    };

                                    Some((manga_id.clone(), overlay))
                                },
                            );
                            let v = stream::iter(reqs)
                                .buffer_unordered(10)
                                .collect::<Vec<_>>()
                                .await
                                .into_iter()
                                .flatten()
                                .collect::<HashMap<_, _>>();

                            Arc::new(v)
                        };

                        self.img = Some(ThreadHandler::new_async(req))
                    }
                } else {
                    data.display_error(ui)
                }
            }
        });
    }
}

impl HomePage {
    fn show(&mut self, ui: &mut Ui, data: &HomeResponse, imgs: &HashMap<String, ImageOverlay>) {
        show_top_bar(ui, HomePages::Home);
        ScrollArea::vertical().show(ui, |ui| {
            show_row(&data.newest, "Newest", ui, imgs);
            show_row(&data.trending, "Trending", ui, imgs);
            show_row(&data.reading, "Reading", ui, imgs);
            show_row(&data.favorites, "Favorites", ui, imgs);
            show_row(&data.latest_updates, "Latest Updates", ui, imgs);
        });
    }
}

fn show_top_bar(ui: &mut Ui, active: HomePages) {
    let app = get_app_data();
    let role = app.get_user_data().unwrap().role;
    let add_button = matches!(
        role,
        Role::Admin | Role::CoAdmin | Role::Moderator | Role::Author
    );
    ui.horizontal(|ui| {
        for p in HomePages::all() {
            if active == p {
                let _ = ui.selectable_label(true, p.to_string());
            } else {
                if ui.button(p.to_string()).clicked() {
                    p.switch_window();
                }
            }
        }

        if add_button && ui.button("Add Manga").clicked() {
            app.open(Page::AddManga)
        }

        if ui.button("Logout").clicked() {
            app.logout()
        }
    });
}

fn show_row(
    item: &[SearchResponse],
    label: &str,
    ui: &mut Ui,
    imgs: &HashMap<String, ImageOverlay>,
) {
    let app = get_app_data();
    let v = item
        .iter()
        .map(|v| {
            let id = v.manga_id.clone();
            let title = app.get_title(&v.titles);
            let img = imgs.get(&id).cloned();
            (id, title, img)
        })
        .collect::<Vec<_>>();
    if !v.is_empty() {
        ui.horizontal(|ui| {
            render_row(label, ui);
        });
        scrollable_items(v, ui, label);
    }
}

fn render_row(label: &str, ui: &mut Ui) {
    ui.label(label);
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        if ui.add(Button::new("More").sense(Sense::click())).clicked() {
            let (search, order, desc) = match label {
                "Newest" => (vec![], Order::Id, true),
                "Trending" => {
                    unimplemented!()
                }
                "Reading" => (vec![], Order::LastRead, true),
                "Favorites" => (
                    vec![ItemOrArray::Item(Item {
                        not: false,
                        data: ItemData::enum_("Favorites"),
                    })],
                    Order::Alphabetical,
                    false,
                ),
                "Latest Updates" => (vec![], Order::Updated, true),
                _ => unreachable!(),
            };

            *get_app_data().search.lock().unwrap() = SearchRequest {
                order,
                desc,
                limit: 20,
                page: 1,
                query: ItemOrArray::Array(Array {
                    or: false,
                    items: search,
                }),
            };
            get_app_data().open(Page::Search)
        }
    });
}

fn item() {}

fn scrollable_items(items: Vec<(String, String, Option<ImageOverlay>)>, ui: &mut Ui, id: &str) {
    ScrollArea::horizontal()
        .auto_shrink([true; 2])
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
        .id_source(id)
        .show(ui, |ui| {
            Grid::new(format!("item_grid_{}", id))
                .num_columns(items.len())
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for (id, text, image) in items {
                        ui.vertical(|ui| {
                            if let Some(img) = image {
                                if ui.add(img).clicked() {
                                    get_app_data().open(Page::MangaInfo(id.clone()))
                                }
                            } else {
                                let (rect, _) =
                                    ui.allocate_exact_size(vec2(200., 300.), Sense::hover());
                                let spinner = Spinner::new();

                                ui.put(rect, spinner);
                            }

                            let v = ui.allocate_exact_size(vec2(200., 40.), Sense::hover());
                            let label = Label::new(text).sense(Sense::click());
                            if ui.put(v.0, label).clicked() {
                                get_app_data().open(Page::MangaInfo(id))
                            }
                        });
                    }
                });
        });
}

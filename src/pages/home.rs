use crate::fetcher::{Complete, Fetcher};
use crate::get_app_data;
use crate::widgets::home_page_swithcer::HomePages;
use crate::window_storage::Page;
use api_structure::auth::role::Role;
use api_structure::home::HomeResponse;
use api_structure::search::{
    Array, Item, ItemData, ItemOrArray, Order, SearchRequest, SearchResponse,
};
use api_structure::RequestImpl;
use eframe::{App, Frame};
use egui::scroll_area::ScrollBarVisibility;
use egui::{
    vec2, Align, Button, Context, Grid, Image, Label, Layout, ScrollArea, Sense, Spinner, Ui,
};
use ethread::ThreadHandler;
use std::collections::HashMap;
use std::fmt::Display;

pub struct HomePage {
    data: Fetcher<HomeResponse>,
    img: Option<ThreadHandler<HashMap<String, Image<'static>>>>,
}

impl Default for HomePage {
    fn default() -> Self {
        Self {
            data: Fetcher::new(HomeResponse::request(&get_app_data().url).unwrap()),
            img: None,
        }
    }
}

impl App for HomePage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(data) = self.data.result() {
                if let Complete::Json(v) = data {
                } else {
                    data.display_error(ui)
                }
            }
        });
    }
}

impl HomePage {
    fn show(&mut self, ui: &mut Ui, data: &HomeResponse, imgs: &HashMap<String, Image<'static>>) {
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
    imgs: &HashMap<String, Image<'static>>,
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
                "Reading" => (
                    vec![ItemOrArray::Item(Item {
                        data: ItemData::enum_("Reading"),
                        not: false,
                    })],
                    Order::LastRead,
                    true,
                ),
                "Favorites" => (
                    vec![ItemOrArray::Item(Item {
                        data: ItemData::enum_("Favorite"),
                        not: false,
                    })],
                    Order::Alphabetical,
                    true,
                ),
                "Latest Updates" => (vec![], Order::Updated, false),
                _ => unreachable!(),
            };

            *get_app_data().search.lock().unwrap() = SearchRequest {
                order,
                desc,
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

fn scrollable_items(items: Vec<(String, String, Option<Image<'static>>)>, ui: &mut Ui, id: &str) {
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

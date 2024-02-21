use crate::fetcher::Fetcher;
use crate::get_app_data;
use crate::window_storage::Page;
use api_structure::auth::jwt::JWTs;
use api_structure::RequestImpl;
use eframe::{App, Frame};
use egui::epaint::{PathShape, TextShape};
use egui::{
    include_image, pos2, vec2, Color32, Context, FontId, Image, Pos2, Rect, Response, Stroke, Ui,
    Vec2, Widget,
};
use std::f64::consts::PI;
use std::ops::Add;

pub struct PlaygroundPage {
    image: Image<'static>,
}

impl Default for PlaygroundPage {
    fn default() -> Self {
        Self {
            image: Image::new(include_image!("../assets/55_2.webp")),
        }
    }
}

impl App for PlaygroundPage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Playground");
            get_app_data().change(Page::SignIn, Page::all())
        });
    }
}

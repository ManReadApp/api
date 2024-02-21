use crate::get_app_data;
use eframe::{App, Frame};
use egui::Context;

#[derive(Default)]
pub struct HomePage {}

impl App for HomePage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Home");
            if ui.button("Logout").clicked() {
                get_app_data().logout()
            }
        });
    }
}

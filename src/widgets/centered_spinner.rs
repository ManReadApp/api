use eframe::emath::Align;
use egui::{Direction, Layout, Ui};

pub fn render(ui: &mut Ui) {
    let layout = Layout {
        main_dir: Direction::TopDown,
        main_wrap: false,
        main_align: Align::Center, // looks best to e.g. center text within a button
        main_justify: true,
        cross_align: Align::Center,
        cross_justify: false,
    };
    ui.with_layout(layout, |ui| {
        ui.spinner();
    });
}

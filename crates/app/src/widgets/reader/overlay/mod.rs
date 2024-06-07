use crate::widgets::reader::overlay::layout::generate_layout;
use eframe::emath::{vec2, Align2, Rect, Vec2};
use eframe::epaint::Color32;
use egui::{Image, Label, Layout, Ui, WidgetText};
use std::collections::HashMap;

mod calculate_size;
mod layout;

const TEXT_SHADOWS: [[f32; 2]; 16] = [
    [1.5 * 1., 1.5 * 0.],
    [1.5 * 0.924, 1.5 * 0.383],
    [1.5 * 0.707, 1.5 * 0.707],
    [1.5 * 0.383, 1.5 * 0.924],
    [1.5 * 0., 1.5 * 1.],
    [1.5 * -0.383, 1.5 * 0.924],
    [1.5 * -0.707, 1.5 * 0.707],
    [1.5 * -0.924, 1.5 * 0.3827],
    [1.5 * -1., 1.5 * 0.],
    [1.5 * -0.924, 1.5 * -0.383],
    [1.5 * -0.707, 1.5 * -0.707],
    [1.5 * -0.383, 1.5 * -0.924],
    [1.5 * 0., 1.5 * -1.],
    [1.5 * 0.383, 1.5 * -0.924],
    [1.5 * 0.707, 1.5 * -0.707],
    [1.5 * 0.924, 1.5 * -0.383],
];

pub struct ReaderTranslationArea {
    pub translated_text: HashMap<String, String>,
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
    pub text_color: Color32,
    pub outline_color: Color32,
    pub background: Image<'static>,
}

pub fn render_overlay(items: &Vec<ReaderTranslationArea>, start: Vec2, scale: f32, ui: &mut Ui) {
    let my_put = |ui: &mut Ui, rect: Rect, layout: Layout, text| {
        ui.allocate_ui_at_rect(rect, |ui| ui.with_layout(layout, |ui| ui.add(text)).inner)
            .inner
    };
    let layout = generate_layout(Align2::CENTER_CENTER);
    for item in items {
        let min = vec2(item.min_x as f32, item.min_y as f32) * scale + start;
        let max = vec2(item.max_x as f32, item.max_y as f32) * scale + start;
        let rect = Rect::from_min_max(min.to_pos2(), max.to_pos2());
        item.background.paint_at(ui, rect);

        let text = item.get_size(rect, ui);
        TEXT_SHADOWS.iter().for_each(|offset| {
            let data = generate_label_data(text.clone(), rect, offset, Some(item.outline_color));
            my_put(ui, data.0, layout, data.1);
        });
        let data = generate_label_data(text, rect, &[0.0; 2], None);
        my_put(ui, data.0, layout, data.1);
    }
}

fn generate_label_data(
    mut widget_text: WidgetText,
    mut rect: Rect,
    offset: &[f32; 2],
    color: Option<Color32>,
) -> (Rect, Label) {
    if let Some(color) = color {
        if let WidgetText::LayoutJob(lj) = &mut widget_text {
            lj.sections.get_mut(0).unwrap().format.color = color;
        }
    }
    rect = rect.translate(Vec2::from(offset));
    (rect, Label::new(widget_text))
}

use egui::scroll_area::ScrollBarVisibility;
use egui::{Rounding, ScrollArea, Ui};
use egui_extras::{Size, StripBuilder};

const BORDER_RADIUS: f32 = 4.0;
const OUTER_MARGIN: f32 = 10.0;

//TODO: add drop shadow
pub trait HoverBackground {
    fn inner(&mut self, ui: &mut Ui, ctx: &egui::Context);
    fn hover_box(&mut self, ui: &mut Ui, ctx: &egui::Context, height: Option<f32>) -> f32 {
        let max_height = ui.available_height();
        let height = height.unwrap_or(max_height).min(max_height);
        let mut out = 0.0;
        StripBuilder::new(ui)
            .size(Size::remainder())
            .size(Size::relative(1.0).at_most(350.0))
            .size(Size::remainder())
            .horizontal(|mut strip| {
                strip.empty();
                strip.strip(|builder| {
                    builder
                        .size(Size::remainder())
                        .size(Size::relative(1.).at_most(height))
                        .size(Size::remainder())
                        .vertical(|mut strip| {
                            strip.empty();

                            strip.cell(|ui| {
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    Rounding::same(BORDER_RADIUS),
                                    ui.style().visuals.panel_fill,
                                );

                                egui::Frame::none()
                                    .outer_margin(OUTER_MARGIN)
                                    .show(ui, |ui| {
                                        ScrollArea::vertical()
                                            .scroll_bar_visibility(
                                                ScrollBarVisibility::AlwaysHidden,
                                            )
                                            .show(ui, |ui| {
                                                out =
                                                    ui.vertical(|ui| {
                                                        self.inner(ui, ctx);
                                                    })
                                                    .response
                                                    .rect
                                                    .size()
                                                    .y + OUTER_MARGIN * 2.0;
                                            });
                                    });
                            });
                            strip.empty();
                        });
                });

                strip.empty();
            });
        if out != height {
            ctx.request_repaint();
        }
        out.min(max_height)
    }
}

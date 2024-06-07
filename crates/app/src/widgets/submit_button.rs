use egui::{vec2, Button, Rect, Response, Sense, Spinner, Ui};

pub fn render(ui: &mut Ui, loading: bool, mut clickable: bool) -> Response {
    if loading {
        clickable = false;
    }
    let text = match loading {
        true => "",
        false => "Submit",
    };
    let sense = match clickable {
        true => Sense::click(),
        false => Sense::hover(),
    };
    let btn = Button::new(text)
        .min_size(vec2(ui.available_width(), 40.))
        .sense(sense);
    let btn_response = ui.add(btn);
    if loading {
        let spinner = Spinner::new();
        let pos =
            (btn_response.rect.max - btn_response.rect.min) / 2. + btn_response.rect.min.to_vec2();
        ui.put(
            Rect::from_center_size(pos.to_pos2(), vec2(32.0, 32.0)),
            spinner,
        );
    }
    btn_response
}

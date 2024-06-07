use eframe::emath::{Align, Align2};
use egui::{Direction, Layout};

pub fn generate_layout(align: Align2) -> Layout {
    match align {
        Align2::LEFT_TOP => Layout::left_to_right(Align::LEFT).with_main_wrap(true),
        Align2::LEFT_CENTER => Layout {
            main_dir: Direction::BottomUp,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: true,
            cross_align: Align::LEFT,
            cross_justify: false,
        },
        Align2::LEFT_BOTTOM => Layout {
            main_dir: Direction::BottomUp,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: false,
            cross_align: Align::LEFT,
            cross_justify: false,
        },
        Align2::RIGHT_TOP => Layout::right_to_left(Align::TOP).with_main_wrap(true),
        Align2::RIGHT_CENTER => Layout::right_to_left(Align::Center).with_main_wrap(true),
        Align2::RIGHT_BOTTOM => Layout::right_to_left(Align::BOTTOM).with_main_wrap(true),
        Align2::CENTER_TOP => Layout {
            main_dir: Direction::TopDown,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: false,
            cross_align: Align::Center,
            cross_justify: false,
        },
        Align2::CENTER_CENTER => Layout {
            main_dir: Direction::TopDown,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: true,
            cross_align: Align::Center,
            cross_justify: false,
        },
        Align2::CENTER_BOTTOM => Layout {
            main_dir: Direction::BottomUp,
            main_wrap: true,
            main_align: Align::Center,
            main_justify: false,
            cross_align: Align::Center,
            cross_justify: false,
        },
    }
}

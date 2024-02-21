#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::app::TemplateApp;
use crate::data::shared_data::SharedData;
use egui::{include_image, Image};
use ethread::ThreadHandler;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Runtime;

mod app;
mod data;
mod fetcher;
mod fonts;
mod pages;
mod util;
mod widgets;
mod window_storage;

static mut APP_DATA: Option<Arc<SharedData>> = None;
fn get_app_data() -> &'static Arc<SharedData> {
    unsafe { APP_DATA.as_ref().unwrap() }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> eframe::Result<()> {
    unsafe { APP_DATA = Some(Arc::new(SharedData::new())) };
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([380.0, 400.0])
            .with_decorations(false)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/logo.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "ManRead",
        native_options,
        Box::new(|_| Box::<TemplateApp>::default()),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    unsafe { APP_DATA = Some(Arc::new(SharedData::new())) };
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::<TemplateApp>::default()),
            )
            .await
            .expect("failed to start eframe");
    });
}

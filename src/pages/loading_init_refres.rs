use crate::data::user::User;
use crate::get_app_data;
use crate::widgets::centered_spinner;
use crate::window_storage::Page;
use eframe::{App, Frame};
use egui::{Align, Context, Direction, Layout};
use ethread::ThreadHandler;
use log::info;

pub struct LoadingPage {
    data: ThreadHandler<Option<User>>,
    ctx_set: bool,
}

impl LoadingPage {
    pub fn new() -> Self {
        let page = async {
            let token = User::load_token();
            info!("Loading token");
            if let Ok(token) = token {
                info!("Found token");
                let tokens = User::get_updated_tokens(&token).await;
                info!("updated token");
                if let Some(jwts) = tokens {
                    return User::new(jwts);
                }
            }
            let _ = User::delete_token();
            None
        };
        Self {
            data: ThreadHandler::new_async(page),
            ctx_set: false,
        }
    }
}

impl App for LoadingPage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if !self.ctx_set {
            self.ctx_set = true;
            self.data.set_context(ctx.clone());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(v) = self.data.task.ready() {
                match v {
                    None => {
                        get_app_data().change(Page::Login, Page::all());
                        ctx.request_repaint();
                        return;
                    }
                    Some(v) => {
                        let data = get_app_data();
                        data.set_user_data(v.clone());
                        data.change(Page::Home, Page::all());
                        ctx.request_repaint();
                        return;
                    }
                }
            }
            centered_spinner::render(ui)
        });
    }
}

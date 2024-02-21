use crate::fetcher::{Complete, Fetcher};
use crate::get_app_data;
use crate::pages::auth::background;
use crate::util::validator::{validate_email, validate_password, validate_username};
use crate::widgets::hover_brackground::HoverBackground;
use crate::widgets::or_continue_with::centered_line_with_text_widget;
use crate::widgets::submit_button;
use api_structure::auth::jwt::JWTs;
use api_structure::auth::login::{
    LoginRequest, LoginWithEmailAndPassword, LoginWithUsernameAndPassword,
};
use api_structure::RequestImpl;
use eframe::{App, Frame};
use egui::{vec2, Align, Context, Image, ImageButton, Label, Layout, Link, Sense, TextEdit, Ui};

pub struct LoginPage {
    height: Option<f32>,
    bg: Image<'static>,
    gh: ImageButton<'static>,
    apple: ImageButton<'static>,
    google: ImageButton<'static>,
    email_login: bool,
    username: String,
    password: String,
    request: Fetcher<JWTs>,
}
impl Default for LoginPage {
    fn default() -> Self {
        //TODO: icons look bad
        Self {
            height: None,
            bg: Image::new(egui::include_image!("../../background.png")),
            gh: ImageButton::new(
                Image::new(egui::include_image!("../assets/logos/github.png"))
                    .fit_to_exact_size(vec2(32., 32.)),
            ),
            apple: ImageButton::new(
                Image::new(egui::include_image!("../assets/logos/apple.png"))
                    .fit_to_exact_size(vec2(32., 32.)),
            ),
            google: ImageButton::new(
                Image::new(egui::include_image!("../assets/logos/google.png"))
                    .fit_to_exact_size(vec2(32., 32.)),
            ),
            email_login: false,
            username: "".to_string(),
            password: "".to_string(),
            request: Fetcher::new(LoginRequest::request(&get_app_data().url).unwrap()),
        }
    }
}

impl App for LoginPage {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        let panel = egui::CentralPanel::default();
        panel.show(ctx, |ui| {
            background(&self.bg, ui);
            self.height = Some(self.hover_box(ui, ctx, self.height));
        });
    }
}

impl HoverBackground for LoginPage {
    fn inner(&mut self, ui: &mut Ui, _: &Context) {
        ui.vertical_centered(|ui| {
            if let Some(res) = self.request.result() {
                match res {
                    Complete::Json(_) => {
                        //TODO: login
                    }
                    _ => res.display_error(ui),
                }
            }
            self.header(ui);
            self.username_field(ui);
            self.password_field(ui);
            self.forgot_password(ui);
            self.submit_button(ui);
            self.or_continue_with(ui);
            self.not_a_member(ui);
        });
    }
}

impl LoginPage {
    fn header(&mut self, ui: &mut Ui) {
        ui.heading("Welcome back");
        ui.add_space(4.0);
        if ui
            .add(Label::new("We've missed you").sense(Sense::click()))
            .on_hover_text("Click to switch to email login")
            .clicked()
        {
            self.email_login = !self.email_login;
        }
        ui.add_space(4.0);
    }

    fn username_field(&mut self, ui: &mut Ui) {
        let mut hint = "Username";
        if self.email_login {
            hint = "Email";
        }
        ui.add(
            TextEdit::singleline(&mut self.username)
                .hint_text(hint)
                .margin(vec2(0., 20.))
                .desired_width(f32::INFINITY),
        );
        ui.add_space(4.0);
    }

    fn password_field(&mut self, ui: &mut Ui) {
        ui.add(
            TextEdit::singleline(&mut self.password)
                .hint_text("Password")
                .margin(vec2(0., 20.))
                .password(true)
                .desired_width(f32::INFINITY),
        );
    }

    fn forgot_password(&self, ui: &mut Ui) {
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            if ui
                .add(Label::new("Forgot password?").sense(Sense::click()))
                .clicked()
            {
                //TODO: change page
                //replace_page(&self.gd.page_management, vec![], Page::ForgotPassword);
            }
        });
    }

    fn submit_button(&mut self, ui: &mut Ui) {
        let loading = self.request.loading();
        let email_validation = match self.email_login {
            true => validate_email(&self.username),
            false => true,
        };
        let username_validation = match self.email_login {
            true => true,
            false => validate_username(&self.username),
        };
        let password_validation = validate_password(&self.password);
        let btn = submit_button::render(
            ui,
            loading,
            email_validation && username_validation && password_validation && !loading,
        );
        if btn.clicked() {
            self.request.set_body(match self.email_login {
                true => LoginRequest::Email(LoginWithEmailAndPassword {
                    email: self.username.clone(),
                    password: self.password.clone(),
                }),
                false => LoginRequest::Username(LoginWithUsernameAndPassword {
                    username: self.username.clone(),
                    password: self.password.clone(),
                }),
            });
            self.request.send()
        }
    }
    fn or_continue_with(&self, ui: &mut Ui) {
        centered_line_with_text_widget(ui, "or continue with");
        let style = ui.style().spacing.icon_spacing;

        ui.add_space(8.0);

        // Icon buttons
        ui.horizontal(|ui| {
            let space = (ui.available_width() - (32 * 3) as f32 - style * 10.) / 2.0;
            ui.add_space(space);
            //TODO: add login with apple, google, facebook, twitter, github, gitlab
            ui.add(self.apple.clone());
            ui.add(self.google.clone());
            ui.add(self.gh.clone());
            ui.add_space(space);
        });
    }

    fn not_a_member(&self, ui: &mut Ui) {
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.label("Not a member?");
            if ui
                .add(Link::new("Register here"))
                .on_hover_text("Click to register")
                .clicked()
            {
                //TODO: change page
            }
        });
    }
}

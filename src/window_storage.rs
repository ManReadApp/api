use crate::get_app_data;
use crate::pages::auth::reset_password::ResetPasswordPage;
use crate::pages::auth::sign_in::LoginPage;
use crate::pages::auth::sign_up::SignUpPage;
use crate::pages::auth::sign_up_info::SignUpInfoPage;
use crate::pages::auth::verify_account::VerifyAccountPage;
use crate::pages::{HomePage, LoadingInitRefreshPage, PlaygroundPage};
use eframe::App;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Page {
    LoadingInitRefresh,
    Home,
    SignIn,
    SignUp,
    SignUpInfo,
    ResetPassword,
    VerifyAccount,
    #[cfg(feature = "dev")]
    Playground,
}

impl Page {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Home,
            Self::SignIn,
            Self::LoadingInitRefresh,
            Self::SignUp,
            Self::SignUpInfo,
            Self::ResetPassword,
            #[cfg(feature = "dev")]
            Self::Playground,
            Self::VerifyAccount,
        ]
    }
}

#[derive(Default)]
pub struct Windows {
    home: Option<HomePage>,
    sign_in: Option<LoginPage>,
    loading: Option<LoadingInitRefreshPage>,
    sign_up: Option<SignUpPage>,
    sign_up_info: Option<SignUpInfoPage>,
    reset_password: Option<ResetPasswordPage>,
    verfiy_account: Option<VerifyAccountPage>,
    #[cfg(feature = "dev")]
    playground: Option<PlaygroundPage>,
}

impl Windows {
    fn dispose(&mut self, page: Page) {
        match page {
            Page::LoadingInitRefresh => self.loading = None,
            Page::Home => self.home = None,
            Page::SignIn => self.sign_in = None,
            Page::SignUp => self.sign_up = None,
            Page::SignUpInfo => self.sign_up_info = None,
            Page::ResetPassword => self.reset_password = None,
            Page::VerifyAccount => self.verfiy_account = None,
            #[cfg(feature = "dev")]
            Page::Playground => self.playground = None,
        };
    }

    pub fn get_app(&mut self, page: Page) -> &mut dyn App {
        match page {
            Page::LoadingInitRefresh => {
                self.loading.get_or_insert_with(LoadingInitRefreshPage::new) as &mut dyn App
            }
            Page::Home => self.home.get_or_insert_with(HomePage::default) as &mut dyn App,
            Page::SignIn => self.sign_in.get_or_insert_with(LoginPage::default) as &mut dyn App,
            Page::SignUp => self.sign_up.get_or_insert_with(SignUpPage::default) as &mut dyn App,
            Page::SignUpInfo => {
                if self.sign_up_info.is_none() {
                    if let Some(sup) = &self.sign_up {
                        if let Some(resp) = &sup.thumb {
                            if let Some((Some(fname), img)) = resp.task.ready() {
                                let name = fname.first().unwrap().1.clone();
                                self.sign_up_info = Some(SignUpInfoPage::new(
                                    sup.email1.clone(),
                                    sup.username.clone(),
                                    sup.password1.clone(),
                                    name,
                                    img.clone(),
                                ));
                            }
                        }
                    }
                }
                if self.sign_up_info.is_none() {
                    get_app_data().open(Page::SignUp);
                    self.sign_up.get_or_insert_with(SignUpPage::default) as &mut dyn App
                } else {
                    self.sign_up_info.as_mut().unwrap() as &mut dyn App
                }
            }
            Page::ResetPassword => {
                self.reset_password
                    .get_or_insert_with(ResetPasswordPage::default) as &mut dyn App
            }
            #[cfg(feature = "dev")]
            Page::Playground => {
                self.playground.get_or_insert_with(PlaygroundPage::default) as &mut dyn App
            }
            Page::VerifyAccount => {
                self.verfiy_account
                    .get_or_insert_with(VerifyAccountPage::default) as &mut dyn App
            }
        }
    }

    pub fn dispose_many(&mut self, pages: HashSet<Page>) {
        for page in pages {
            self.dispose(page);
        }
    }
}

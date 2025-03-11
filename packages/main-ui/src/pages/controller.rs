#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::Language;
use models::*;

use crate::{
    routes::Route, service::login_service::use_login_service, utils::hash::get_hash_string,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    email: Signal<String>,
    password: Signal<String>,
    not_matched_error: Signal<bool>,
    not_exists_error: Signal<bool>,
    login_failed_error: Signal<bool>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            email: use_signal(|| "".to_string()),
            password: use_signal(|| "".to_string()),
            not_matched_error: use_signal(|| false),
            not_exists_error: use_signal(|| false),
            login_failed_error: use_signal(|| false),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_not_matched_error(&self) -> bool {
        (self.not_matched_error)()
    }

    pub fn get_exists_error(&self) -> bool {
        (self.not_exists_error)()
    }

    pub fn get_login_failed_error(&self) -> bool {
        (self.login_failed_error)()
    }

    pub fn get_email(&self) -> String {
        (self.email)()
    }

    pub fn get_password(&self) -> String {
        (self.password)()
    }

    pub fn set_email(&mut self, email: String) {
        self.email.set(email);
    }

    pub fn set_password(&mut self, password: String) {
        self.password.set(password);
    }

    pub async fn login_clicked(&mut self, lang: Language) {
        let user_api = User::get_client(&crate::config::get().api_url);
        let mut login_service = use_login_service();
        let navigator = use_navigator();
        let res = user_api
            .login(
                self.get_email(),
                get_hash_string(self.get_password().as_bytes()),
            )
            .await;
        match res {
            Ok(user) => {
                btracing::debug!("{:?}", user);
                let token = rest_api::get_authz_token().unwrap_or_default();
                login_service.setup(self.get_email(), token).await;
                login_service.set_orgs(user.orgs);
                navigator.push(Route::SurveyPage { lang });
            }
            Err(e) => match e {
                ApiError::AuthKeyNotMatch(e) => {
                    btracing::debug!("{:?}", e);
                    self.not_matched_error.set(true);
                }
                _ => {
                    btracing::debug!("{:?}", e);
                    self.login_failed_error.set(true);
                }
            },
        }
    }
}

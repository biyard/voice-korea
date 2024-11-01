#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    api::v1::users::login::{login_user, LoginUserRequest},
    service::login_service::use_login_service,
};

use super::{Language, Route};

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
        let navigator = use_navigator();
        let res = login_user(LoginUserRequest {
            email: self.get_email(),
            password: self.get_password(),
        })
        .await;

        match res {
            Ok(_) => {
                use_login_service().setup(self.get_email());
                navigator.push(Route::DashboardPage { lang });
            }
            Err(e) => match e {
                ServerFnError::ServerError(v) => {
                    if v == "not matched" {
                        self.not_matched_error.set(true);
                    } else if v == "not exists user" {
                        self.not_exists_error.set(true);
                    } else {
                        self.login_failed_error.set(true);
                    }
                }
                _ => {}
            },
        }
    }
}

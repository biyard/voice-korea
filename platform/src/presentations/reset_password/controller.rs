#![allow(non_snake_case)]
use dioxus::prelude::*;
use regex::Regex;

use crate::api::{
    aws::ses::send_email,
    v1::users::{
        reset::{reset_password, ResetPasswordRequest},
        verify::{verify_email, VerifyEmailRequest},
    },
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    email: Signal<String>,
    name: Signal<String>,
    phone_number: Signal<String>,
    authentication_number: Signal<String>,
    step: Signal<u64>,
    new_password: Signal<String>,
    new_password_check: Signal<String>,

    email_address_error: Signal<bool>,
    not_exists_email_error: Signal<bool>,
    invalid_authkey_error: Signal<bool>,
    unknown_error: Signal<bool>,
    password_error: Signal<bool>,
    password_check_error: Signal<bool>,
    password_pattern_error: Signal<bool>,
    password_unknown_error: Signal<bool>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            email: use_signal(|| "".to_string()),
            name: use_signal(|| "".to_string()),
            phone_number: use_signal(|| "".to_string()),
            authentication_number: use_signal(|| "".to_string()),
            step: use_signal(|| 0),
            new_password: use_signal(|| "".to_string()),
            new_password_check: use_signal(|| "".to_string()),
            email_address_error: use_signal(|| false),
            not_exists_email_error: use_signal(|| false),
            invalid_authkey_error: use_signal(|| false),
            unknown_error: use_signal(|| false),
            password_error: use_signal(|| false),
            password_check_error: use_signal(|| false),
            password_pattern_error: use_signal(|| false),
            password_unknown_error: use_signal(|| false),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_email(&self) -> String {
        (self.email)()
    }

    pub fn get_name(&self) -> String {
        (self.name)()
    }

    pub fn get_phone_number(&self) -> String {
        (self.phone_number)()
    }

    pub fn get_authentication_number(&self) -> String {
        (self.authentication_number)()
    }

    pub fn get_step(&self) -> u64 {
        (self.step)()
    }

    pub fn get_new_password(&self) -> String {
        (self.new_password)()
    }

    pub fn get_new_password_check(&self) -> String {
        (self.new_password_check)()
    }

    pub fn get_email_address_error(&self) -> bool {
        (self.email_address_error)()
    }

    pub fn get_invalid_authkey_error(&self) -> bool {
        (self.invalid_authkey_error)()
    }

    pub fn get_unknown_error(&self) -> bool {
        (self.unknown_error)()
    }

    pub fn get_password_error(&self) -> bool {
        (self.password_error)()
    }

    pub fn get_password_check_error(&self) -> bool {
        (self.password_check_error)()
    }

    pub fn get_password_pattern_error(&self) -> bool {
        (self.password_pattern_error)()
    }

    pub fn get_password_unknown_error(&self) -> bool {
        (self.password_unknown_error)()
    }

    pub fn get_not_exists_email_error(&self) -> bool {
        (self.not_exists_email_error)()
    }

    pub fn set_step(&mut self, step: u64) {
        self.step.set(step);
    }

    pub fn set_email(&mut self, email: String) {
        self.email.set(email);
    }

    pub fn set_name(&mut self, name: String) {
        self.name.set(name);
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number.set(phone_number);
    }

    pub fn set_authentication_number(&mut self, authentication_number: String) {
        self.authentication_number.set(authentication_number);
    }

    pub fn set_new_password(&mut self, new_password: String) {
        self.new_password.set(new_password);
    }

    pub fn set_new_password_check(&mut self, new_password_check: String) {
        self.new_password_check.set(new_password_check);
    }

    pub async fn set_click_send_authentication(&mut self) {
        let re = Regex::new(r"^[a-zA-Z0-9+-\_.]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

        if !re.is_match(self.get_email().as_str()) {
            self.email_address_error.set(true);
            return;
        }

        self.email_address_error.set(false);
        let _ = send_email(vec![self.get_email()]).await;
    }

    pub async fn clicked_email_authentication(&mut self) {
        let res = verify_email(VerifyEmailRequest {
            email: self.get_email(),
            verification_code: self.get_authentication_number(),
        })
        .await;

        match res {
            Ok(_) => {
                self.invalid_authkey_error.set(false);
                self.set_step(1);
            }
            Err(e) => match e {
                ServerFnError::ServerError(v) => {
                    if v == "Auth key is not exists" {
                        self.invalid_authkey_error.set(true);
                    } else if v == "Email is not exists" {
                        self.not_exists_email_error.set(true);
                    } else {
                        self.unknown_error.set(true);
                    }
                }
                _ => {}
            },
        }
    }

    pub async fn clicked_reset_new_password(&mut self) {
        let mut has_number = false;
        let mut has_special = false;
        let mut has_alpha = false;

        for c in self.get_new_password().chars() {
            if c.is_numeric() {
                has_number = true;
            } else if c.is_alphabetic() {
                has_alpha = true;
            } else {
                has_special = true;
            }
        }
        if self.get_new_password().is_empty() {
            self.password_error.set(true);
            return;
        } else if self.get_new_password() != self.get_new_password_check() {
            self.password_check_error.set(true);
            return;
        } else if !has_number || !has_special || !has_alpha {
            self.password_pattern_error.set(true);
            return;
        }

        let res = reset_password(ResetPasswordRequest {
            email: self.get_email(),
            password: self.get_new_password(),
        })
        .await;

        match res {
            Ok(_) => {
                self.password_error.set(false);
                self.password_check_error.set(false);
                self.password_pattern_error.set(false);
                self.password_unknown_error.set(false);
                self.set_step(2);
            }
            Err(e) => match e {
                ServerFnError::ServerError(_v) => {
                    self.password_unknown_error.set(true);
                }
                _ => {}
            },
        }
    }
}

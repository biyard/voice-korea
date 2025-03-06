#![allow(non_snake_case)]
use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::*;
use regex::Regex;

use crate::utils::hash::get_hash_string;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    email: Signal<String>,
    name: Signal<String>,
    phone_number: Signal<String>,
    authentication_number: Signal<String>,
    auth_key: Signal<String>,
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
    pub fn init(lang: Language) -> Self {
        let ctrl = Self {
            lang,
            email: use_signal(|| "".to_string()),
            name: use_signal(|| "".to_string()),
            phone_number: use_signal(|| "".to_string()),
            authentication_number: use_signal(|| "".to_string()),
            auth_key: use_signal(|| "".to_string()),
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

    pub fn get_auth_key(&self) -> String {
        (self.auth_key)()
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

    pub async fn send_verification_code(&mut self) -> models::Result<()> {
        let re = Regex::new(r"^[a-zA-Z0-9+-\_.]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

        if !re.is_match(self.get_email().as_str()) {
            self.email_address_error.set(true);
            return Err(ApiError::ValidationError(
                "email must be formed as email address".to_string(),
            ));
        }
        self.email_address_error.set(false);

        let endpoint = crate::config::get().api_url;
        Verification::get_client(endpoint)
            .send_verification_code(self.get_email())
            .await?;

        Ok(())
    }

    pub async fn verify_code(&mut self) {
        let endpoint = crate::config::get().api_url;

        match Verification::get_client(endpoint)
            .verify(self.email(), self.authentication_number())
            .await
        {
            Ok(_) => {
                self.invalid_authkey_error.set(false);
                self.unknown_error.set(false);
                self.set_step(1);
            }
            Err(ApiError::InvalidVerificationCode) => {
                self.invalid_authkey_error.set(true);
            }
            Err(e) => {
                btracing::error!("{}", e.translate(&self.lang));
                self.unknown_error.set(true);
            }
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
        let endpoint = crate::config::get().api_url;
        match User::get_client(endpoint)
            .reset(
                self.email(),
                get_hash_string(self.get_new_password().as_bytes()),
                self.authentication_number(),
            )
            .await
        {
            Ok(_) => {
                self.password_error.set(false);
                self.password_check_error.set(false);
                self.password_pattern_error.set(false);
                self.password_unknown_error.set(false);
                self.set_step(2);
            }
            Err(e) => {
                btracing::error!("{}", e.translate(&self.lang));
                self.password_unknown_error.set(true);
            }
        }
    }
}

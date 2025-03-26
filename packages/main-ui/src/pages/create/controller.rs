#![allow(non_snake_case, dead_code)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::*;
use regex::Regex;

use crate::utils::hash::get_hash_string;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    authorize_type: Signal<u64>, //0: 개인, 1: 법인
    step: Signal<u64>,
    company: Signal<String>,
    business_number: Signal<String>,

    extend_first: Signal<bool>,
    extend_second: Signal<bool>,
    extend_third: Signal<bool>,
    click_first: Signal<bool>,
    click_second: Signal<bool>,
    click_third: Signal<bool>,

    email_address: Signal<String>,
    authentication_number: Signal<String>,
    password: Signal<String>,
    password_check: Signal<String>,
    name: Signal<String>,
    cellphone_number: Signal<String>,
    simple_address: Signal<String>,
    detail_address: Signal<String>,
    auth_key: Signal<String>,

    email_address_error: Signal<bool>,
    password_error: Signal<bool>,
    password_check_error: Signal<bool>,
    password_pattern_error: Signal<bool>,
    invalid_authkey_error: Signal<bool>,
    already_exists_user_error: Signal<bool>,
    auth_error: Signal<bool>,
    unknown_error: Signal<bool>,
    // click_send_authentication: Signal<bool>,
    // click_search_address: Signal<bool>,
    // click_complete_join_membership: Signal<bool>,
    auth_api: Signal<VerificationClient>,
    user_api: Signal<UserClient>,
}

impl Controller {
    pub fn init() -> Self {
        let api_url = crate::config::get().api_url;
        let auth_api = use_signal(|| Verification::get_client(api_url));
        let user_api = use_signal(|| User::get_client(api_url));

        let ctrl = Self {
            authorize_type: use_signal(|| 0),
            step: use_signal(|| 0),
            company: use_signal(|| "".to_string()),
            business_number: use_signal(|| "".to_string()),

            extend_first: use_signal(|| false),
            extend_second: use_signal(|| false),
            extend_third: use_signal(|| false),
            click_first: use_signal(|| false),
            click_second: use_signal(|| false),
            click_third: use_signal(|| false),

            email_address: use_signal(|| "".to_string()),
            authentication_number: use_signal(|| "".to_string()),
            password: use_signal(|| "".to_string()),
            password_check: use_signal(|| "".to_string()),
            name: use_signal(|| "".to_string()),
            cellphone_number: use_signal(|| "".to_string()),
            simple_address: use_signal(|| "".to_string()),
            detail_address: use_signal(|| "".to_string()),
            email_address_error: use_signal(|| false),
            password_error: use_signal(|| false),
            password_check_error: use_signal(|| false),
            password_pattern_error: use_signal(|| false),
            invalid_authkey_error: use_signal(|| false),
            already_exists_user_error: use_signal(|| false),
            unknown_error: use_signal(|| false),
            auth_error: use_signal(|| false),

            auth_key: use_signal(|| "".to_string()),
            auth_api,
            user_api,
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_authorize_type(&self) -> u64 {
        (self.authorize_type)()
    }

    pub fn get_step(&self) -> u64 {
        (self.step)()
    }

    pub fn get_company(&self) -> String {
        (self.company)()
    }

    pub fn get_business_number(&self) -> String {
        (self.business_number)()
    }

    pub fn get_extend_first(&self) -> bool {
        (self.extend_first)()
    }

    pub fn get_extend_second(&self) -> bool {
        (self.extend_second)()
    }

    pub fn get_extend_third(&self) -> bool {
        (self.extend_third)()
    }

    pub fn get_click_first(&self) -> bool {
        (self.click_first)()
    }

    pub fn get_click_second(&self) -> bool {
        (self.click_second)()
    }

    pub fn get_click_third(&self) -> bool {
        (self.click_third)()
    }

    pub fn get_email_address(&self) -> String {
        (self.email_address)()
    }

    pub fn get_authentication_number(&self) -> String {
        (self.authentication_number)()
    }

    pub fn get_name(&self) -> String {
        (self.name)()
    }

    pub fn get_cellphone_number(&self) -> String {
        (self.cellphone_number)()
    }

    pub fn get_simple_address(&self) -> String {
        (self.simple_address)()
    }

    pub fn get_detail_address(&self) -> String {
        (self.detail_address)()
    }

    pub fn get_password(&self) -> String {
        (self.password)()
    }

    pub fn get_password_check(&self) -> String {
        (self.password_check)()
    }

    pub fn get_email_address_error(&self) -> bool {
        (self.email_address_error)()
    }

    pub fn get_password_error(&self) -> bool {
        (self.password_error)()
    }

    pub fn get_password_check_error(&self) -> bool {
        (self.password_check_error)()
    }

    pub fn get_invalid_authkey_error(&self) -> bool {
        (self.invalid_authkey_error)()
    }

    pub fn get_authorization_error(&self) -> bool {
        (self.auth_error)()
    }

    pub fn get_already_exists_user_error(&self) -> bool {
        (self.already_exists_user_error)()
    }

    pub fn get_unknown_error(&self) -> bool {
        (self.unknown_error)()
    }

    pub fn get_password_pattern_error(&self) -> bool {
        (self.password_pattern_error)()
    }

    // pub fn get_click_send_authentication(&self) -> bool {
    //     (self.click_send_authentication)()
    // }

    // pub fn get_click_search_address(&self) -> bool {
    //     (self.click_search_address)()
    // }

    // pub fn get_click_complete_join_membership(&self) -> bool {
    //     (self.click_complete_join_membership)()
    // }

    pub fn set_authorize_type(&mut self, authorize_type: u64) {
        self.authorize_type.set(authorize_type);
    }

    pub fn set_step(&mut self, step: u64) {
        self.step.set(step);
    }

    pub fn set_company(&mut self, company: String) {
        self.company.set(company);
    }

    pub fn set_business_number(&mut self, business_number: String) {
        self.business_number.set(business_number);
    }

    pub fn set_extend_first_terms(&mut self, extend: bool) {
        self.extend_first.set(extend);
    }

    pub fn set_extend_second_terms(&mut self, extend: bool) {
        self.extend_second.set(extend);
    }

    pub fn set_extend_third_terms(&mut self, extend: bool) {
        self.extend_third.set(extend);
    }

    pub fn set_click_first_terms(&mut self, clicked: bool) {
        self.click_first.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_click_second_terms(&mut self, clicked: bool) {
        self.click_second.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_click_third_terms(&mut self, clicked: bool) {
        self.click_third.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_email_address(&mut self, email: String) {
        self.email_address.set(email);
    }

    pub fn set_authentication_number(&mut self, authentication_number: String) {
        self.authentication_number.set(authentication_number);
    }

    pub fn set_name(&mut self, name: String) {
        self.name.set(name);
    }

    pub fn set_cellphone_number(&mut self, cellphone_number: String) {
        self.cellphone_number.set(cellphone_number);
    }

    pub fn set_simple_address(&mut self, simple_address: String) {
        self.simple_address.set(simple_address);
    }

    pub fn set_detail_address(&mut self, detail_address: String) {
        self.detail_address.set(detail_address);
    }

    pub fn set_password(&mut self, password: String) {
        self.password.set(password);
    }

    pub fn set_password_check(&mut self, password_check: String) {
        self.password_check.set(password_check);
    }

    pub async fn set_click_send_authentication(&mut self) {
        let re = Regex::new(r"^[a-zA-Z0-9+-\_.]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

        if !re.is_match(self.get_email_address().as_str()) {
            self.email_address_error.set(true);
            return;
        }

        self.email_address_error.set(false);

        match self
            .auth_api
            .read()
            .send_verification_code(self.get_email_address())
            .await
        {
            Ok(Verification { id, .. }) => {
                self.auth_key.set(id.to_string());
            }
            Err(e) => {
                tracing::error!("send email failed: {}", e);
            }
        }
    }

    pub fn get_auth_key(&self) -> String {
        (self.auth_key)()
    }

    pub fn set_click_search_address(&mut self) {
        tracing::info!("search address button clicked");
    }

    pub async fn set_click_complete_join_membership(&mut self) {
        let re = Regex::new(r"^[a-zA-Z0-9+\-_.]{6}$").unwrap();

        let mut has_number = false;
        let mut has_special = false;
        let mut has_alpha = false;

        let mut has_error = false;

        for c in self.get_password().chars() {
            if c.is_numeric() {
                has_number = true;
            } else if c.is_alphabetic() {
                has_alpha = true;
            } else {
                has_special = true;
            }
        }

        if !re.is_match(&self.get_authentication_number().as_str()) {
            self.auth_error.set(true);
            has_error = true;
        }
        if self.get_password().is_empty() {
            self.password_error.set(true);
            has_error = true;
        }
        if self.get_password() != self.get_password_check() {
            self.password_check_error.set(true);
            has_error = true;
        }
        if !has_number || !has_special || !has_alpha {
            self.password_pattern_error.set(true);
            has_error = true;
        }

        if has_error {
            return;
        }

        self.password_error.set(false);
        self.password_check_error.set(false);
        self.password_pattern_error.set(false);
        self.auth_error.set(false);
        let res = self
            .user_api
            .read()
            .signup(
                self.get_email_address(),
                get_hash_string(self.get_password().as_bytes()),
                self.get_authentication_number(),
            )
            .await;

        match res {
            Ok(_) => {
                self.invalid_authkey_error.set(false);
                self.already_exists_user_error.set(false);
                self.unknown_error.set(true);
                self.auth_key.set("".to_string());
                self.set_step(1);
            }
            Err(e) => match e {
                ApiError::InvalidVerificationCode => {
                    self.invalid_authkey_error.set(true);
                }
                ApiError::DuplicateUser => {
                    self.already_exists_user_error.set(true);
                }
                e => {
                    tracing::error!("signup failed: {}", e);
                    self.unknown_error.set(true);
                }
            },
        };
    }

    fn check_and_update_terms_agreement(&mut self) {
        if self.get_click_first() && self.get_click_second() && self.get_click_third() {
            self.set_step(2);
            self.click_first.set(false);
            self.click_second.set(false);
            self.click_third.set(false);
        }
    }
}

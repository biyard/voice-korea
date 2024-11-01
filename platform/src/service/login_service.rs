use dioxus::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct LoginService {
    pub email: Signal<Option<String>>,
}

impl LoginService {
    pub fn init() {
        let srv = LoginService {
            email: use_signal(|| None),
        };

        use_context_provider(|| srv);
    }

    pub fn get_email(&self) -> String {
        match (self.email)() {
            Some(email) => email,
            None => "".to_string(),
        }
    }

    pub fn setup(&mut self, email: String) {
        self.email.set(Some(email));
    }
}

pub fn use_login_service() -> LoginService {
    use_context()
}

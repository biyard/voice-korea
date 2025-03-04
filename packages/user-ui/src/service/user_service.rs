#![allow(non_snake_case)]
use crate::config;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_oauth::prelude::FirebaseService;
use models::Error;

pub enum UserEvent {
    Signup(String, String, String, String),
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    pub firebase: Signal<google_wallet::FirebaseWallet>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
    pub profile_url: Signal<String>,
    pub principal: Signal<String>,
}

impl UserService {
    pub fn init() {
        let firebase = FirebaseService::new(
            config::get().firebase.api_key.clone(),
            config::get().firebase.auth_domain.clone(),
            config::get().firebase.project_id.clone(),
            config::get().firebase.storage_bucket.clone(),
            config::get().firebase.messaging_sender_id.clone(),
            config::get().firebase.app_id.clone(),
            config::get().firebase.measurement_id.clone(),
        );

        use_context_provider(move || firebase);
    }

    pub async fn google_login(&mut self) -> UserEvent {
        tracing::debug!("UserService::login");
        let (evt, principal, email, name, profile_url) = self.request_to_firebase().await.unwrap();
        match evt {
            google_wallet::WalletEvent::Signup => {
                tracing::debug!(
                    "UserService::Signup: email={} name={} profile_url={}",
                    email,
                    name,
                    profile_url
                );

                return UserEvent::Signup(principal, email, name, profile_url);
            }
            google_wallet::WalletEvent::Login => {
                tracing::debug!(
                    "UserService::Login: email={} name={} profile_url={}",
                    email,
                    name,
                    profile_url
                );
                // rest_api::set_signer(Box::new(*self));
                // let cli = (self.cli)();

                // let user: User = match cli.check_email(email.clone()).await {
                //     // Login
                //     Ok(v) => v,
                //     Err(e) => {
                //         // Signup
                //         rest_api::remove_signer();

                //         match e {
                //             ServiceError::NotFound => {
                //                 return UserEvent::Signup(principal, email, name, profile_url);
                //             }
                //             e => {
                //                 tracing::error!("UserService::login: error={:?}", e);
                //                 return UserEvent::Logout;
                //             }
                //         }
                //     }
                // };

                self.email.set(email);
                // self.nickname.set(user.nickname);
                // self.profile_url.set(user.profile_url);
                self.principal.set(principal);

                return UserEvent::Login;
            }
            google_wallet::WalletEvent::Logout => {
                tracing::debug!("UserService::login: SignOut");
            }
        }

        UserEvent::Logout
    }

    async fn request_to_firebase(
        &mut self,
    ) -> Result<(google_wallet::WalletEvent, String, String, String, String), Error> {
        let mut firebase = self.firebase.write();
        let (evt, principal, email, name, profile_url) =
            match firebase.request_wallet_with_google().await {
                Ok(evt) => {
                    tracing::debug!("UserService::login: cred={:?}", evt);
                    let principal = firebase.get_principal();
                    if principal.is_empty() {
                        tracing::error!("UserService::login: principal is empty");
                        return Err(Error::Unauthorized);
                    }

                    let (email, name, profile_url) = match firebase.get_user_info() {
                        Some(v) => v,
                        None => {
                            tracing::error!("UserService::login: None");
                            return Err(Error::Unauthorized);
                        }
                    };

                    (evt, principal, email, name, profile_url)
                }
                Err(e) => {
                    tracing::error!("UserService::login: error={:?}", e);
                    return Err(Error::Unauthorized);
                }
            };

        Ok((evt, principal, email, name, profile_url))
    }
}

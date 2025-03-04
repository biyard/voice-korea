#![allow(non_snake_case)]
use crate::config;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_oauth::prelude::FirebaseService;
use models::{ApiError, Error, ParticipantUser, ParticipantUserClient};
pub enum UserEvent {
    Signup(String, String, String, String),
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserService {
    pub cli: Signal<ParticipantUserClient>,
    pub firebase: Signal<FirebaseService>,
    pub firebase_wallet: Signal<google_wallet::FirebaseWallet>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
    pub profile_url: Signal<String>,
    pub principal: Signal<String>,
}

impl UserService {
    pub fn init() {
        let conf = config::get();

        let firebase = FirebaseService::new(
            conf.firebase.api_key.clone(),
            conf.firebase.auth_domain.clone(),
            conf.firebase.project_id.clone(),
            conf.firebase.storage_bucket.clone(),
            conf.firebase.messaging_sender_id.clone(),
            conf.firebase.app_id.clone(),
            conf.firebase.measurement_id.clone(),
        );

        #[cfg(feature = "web")]
        let firebase_wallet = google_wallet::FirebaseWallet::new(
            conf.firebase.api_key.clone(),
            conf.firebase.auth_domain.clone(),
            conf.firebase.project_id.clone(),
            conf.firebase.storage_bucket.clone(),
            conf.firebase.messaging_sender_id.clone(),
            conf.firebase.app_id.clone(),
            conf.firebase.measurement_id.clone(),
        );

        #[cfg(not(feature = "web"))]
        let firebase_wallet = google_wallet::FirebaseWallet::default();
        let cli = ParticipantUser::get_client(&conf.api_url);

        let user = Self {
            cli: Signal::new(cli),
            firebase: use_signal(|| firebase),
            firebase_wallet: use_signal(|| firebase_wallet),
            email: use_signal(|| "".to_string()),
            nickname: use_signal(|| "".to_string()),
            profile_url: use_signal(|| "".to_string()),
            principal: use_signal(|| "".to_string()),
        };

        use_context_provider(move || firebase);
        use_context_provider(move || user);
    }

    pub async fn google_login(&mut self) -> UserEvent {
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
                rest_api::set_signer(Box::new(*self));
                let cli = (self.cli)();

                let user: ParticipantUser = match cli.check_email(email.clone()).await {
                    // Login
                    Ok(v) => v,
                    Err(e) => {
                        // Signup
                        rest_api::remove_signer();

                        match e {
                            ApiError::NotFound => {
                                return UserEvent::Signup(principal, email, name, profile_url);
                            }
                            e => {
                                tracing::error!("UserService::login: error={:?}", e);
                                return UserEvent::Logout;
                            }
                        }
                    }
                };

                self.email.set(email);
                self.nickname.set(user.nickname);
                self.profile_url.set(user.profile_url);
                self.principal.set(principal);

                return UserEvent::Login;
            }
            google_wallet::WalletEvent::Logout => {
                tracing::debug!("UserService::login: SignOut");
            }
        }

        UserEvent::Logout
    }

    pub async fn login_or_signup(
        &self,
        principal: &str,
        email: &str,
        nickname: &str,
        profile_url: &str,
    ) -> Result<(), Error> {
        rest_api::set_signer(Box::new(*self));

        tracing::debug!(
            "UserService::signup: principal={} email={} nickname={} profile_url={}",
            principal,
            email,
            nickname,
            profile_url
        );

        let cli = (self.cli)();

        let res: ParticipantUser = match cli
            .signup(
                nickname.to_string(),
                email.to_string(),
                profile_url.to_string(),
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("UserService::signup: error={:?}", e);
                rest_api::remove_signer();
                return Err(Error::SignupFailed(e.to_string()));
            }
        };

        tracing::debug!("UserService::signup: user={:?}", res);
        Ok(())
    }

    async fn request_to_firebase(
        &mut self,
    ) -> Result<(google_wallet::WalletEvent, String, String, String, String), Error> {
        let mut firebase = (self.firebase_wallet)();
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

impl rest_api::Signer for UserService {
    fn signer(&self) -> String {
        (self.firebase_wallet)().get_principal()
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("UserService::sign: msg={}", msg);
        let firebase = (self.firebase_wallet)();

        if !firebase.get_login() {
            tracing::debug!("UserService::sign: not login {firebase:?}");
            return Err("not login".to_string().into());
        }

        let sig = firebase.sign(msg);
        if sig.is_none() {
            return Err("signature is none".to_string().into());
        }
        let sig = rest_api::Signature {
            signature: sig.unwrap().as_ref().to_vec(),
            public_key: firebase.public_key().unwrap_or_default(),
            algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
        };

        Ok(sig)
    }
}

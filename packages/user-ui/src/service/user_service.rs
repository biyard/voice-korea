#![allow(non_snake_case)]
use crate::config;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_oauth::prelude::FirebaseService;
use dioxus_translate::Language;
use google_wallet::{drive_api::DriveApi, WalletEvent};
use models::{ApiError, User, UserClient};
pub enum UserEvent {
    Signup(String, String, String),
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserService {
    pub cli: Signal<UserClient>,
    pub firebase: Signal<FirebaseService>,
    pub firebase_wallet: Signal<google_wallet::FirebaseWallet>,

    pub auth_token: Signal<String>,
    pub user_id: Signal<i64>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
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
        let cli = User::get_client(&conf.api_url);

        let user = Self {
            cli: Signal::new(cli),
            firebase: use_signal(|| firebase),
            firebase_wallet: use_signal(|| firebase_wallet),

            auth_token: use_signal(|| "".to_string()),
            user_id: use_signal(|| 0),
            email: use_signal(|| "".to_string()),
            nickname: use_signal(|| "".to_string()),
        };

        use_context_provider(move || firebase);
        use_context_provider(move || user);
    }

    pub async fn google_login(&mut self) -> UserEvent {
        let (evt, token, email, name, profile_url) = self.request_to_firebase().await.unwrap();

        match evt {
            google_wallet::WalletEvent::Signup => {
                tracing::debug!(
                    "UserService::Signup: token={} email={} name={} profile_url={}",
                    token,
                    email,
                    name,
                    profile_url
                );

                self.auth_token.set(token);

                return UserEvent::Signup(email, name, profile_url);
            }
            google_wallet::WalletEvent::Login => {
                tracing::debug!(
                    "UserService::Login: token={} email={} name={} profile_url={}",
                    token,
                    email,
                    name,
                    profile_url
                );
                let cli = (self.cli)();

                let _ = match cli.user_login(email.clone()).await {
                    // Login
                    Ok(v) => {
                        self.email.set(email.clone());
                        self.user_id.set(v.id);
                        self.nickname.set(v.clone().nickname.unwrap_or_default());
                        v
                    }
                    Err(e) => {
                        // Signup
                        match e {
                            ApiError::NotFound => {
                                self.auth_token.set(token);
                                return UserEvent::Signup(email, name, profile_url);
                            }
                            e => {
                                tracing::error!("UserService::login: error={:?}", e);
                                return UserEvent::Logout;
                            }
                        }
                    }
                };

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
        lang: Language,
        email: &str,
        nickname: &str,
    ) -> Result<(), ApiError> {
        tracing::debug!("UserService::signup: email={} nickname={}", email, nickname,);

        let cli = (self.cli)();
        let mut ctrl = self.clone();

        let res: User = match cli
            .user_signup(email.to_string(), Some(nickname.to_string()))
            .await
        {
            Ok(v) => {
                ctrl.email.set(email.to_string());
                ctrl.user_id.set(v.id);
                ctrl.nickname.set(v.clone().nickname.unwrap_or_default());

                v
            }
            Err(e) => {
                btracing::error!("{}", e.translate(&lang).to_string());
                tracing::error!("UserService::signup: error={:?}", e);
                return Err(ApiError::SignupFailed(e.to_string()));
            }
        };

        tracing::debug!("UserService::signup: user={:?}", res);
        Ok(())
    }

    async fn request_to_firebase(
        &mut self,
    ) -> Result<(google_wallet::WalletEvent, String, String, String, String), ApiError> {
        let (evt, token, email, name, profile_url) = match self.handle_google().await {
            Ok(evt) => {
                tracing::debug!("UserService::login: cred={:?}", evt);

                (evt.0, evt.1, evt.3, evt.2, evt.4)
            }
            Err(e) => {
                tracing::error!("UserService::login: error={:?}", e);
                return Err(ApiError::Unauthorized);
            }
        };

        Ok((evt, token, email, name, profile_url))
    }

    pub async fn handle_google(
        &mut self,
    ) -> Result<(WalletEvent, String, String, String, String), String> {
        let cred = (self.firebase)()
            .sign_in_with_popup(vec![
                "https://www.googleapis.com/auth/drive.appdata".to_string()
            ])
            .await;

        tracing::debug!("cred: {:?}", cred);

        let cli = DriveApi::new(cred.access_token.clone());
        let data = match cli.list_files().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("failed to get file {e}");
                return Err("failed to get file".to_string());
            }
        };
        tracing::debug!("data: {data:?}");

        let (evt, _) = match data
            .iter()
            .find(|x| x.name == option_env!("ENV").unwrap_or("local").to_string())
        {
            Some(v) => match cli.get_file(&v.id).await {
                Ok(v) => {
                    tracing::debug!("file content: {v}");

                    (WalletEvent::Login, v)
                }
                Err(e) => {
                    tracing::warn!("failed to get file {e}");

                    return Err("failed to get file".to_string());
                }
            },
            None => {
                let now: DateTime<Utc> = Utc::now();
                tracing::warn!("file not found");
                let timestamp = now.timestamp();

                if let Err(e) = cli.upload_file(&(timestamp.to_string())).await {
                    tracing::error!("failed to upload file {e}");
                    return Err("failed to upload file".to_string());
                };

                (WalletEvent::Signup, format!("google-{timestamp}"))
            }
        };

        Ok((
            evt,
            cred.access_token,
            cred.display_name,
            cred.email,
            cred.photo_url,
        ))
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

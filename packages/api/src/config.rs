use by_types::config::*;

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub database: DatabaseConfig,
    pub verification_expiration: i64,
    pub auth: AuthConfig,
    pub aws: AwsConfig,
    pub from_email: &'static str,
    pub nonce_lab: NonceLabConfig,
    pub server_key: &'static str,
    pub bucket_name: &'static str,
    pub chime_bucket_name: &'static str,
    pub presigned_url_expiration: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct NonceLabConfig {
    pub endpoint: &'static str,
    pub token: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            verification_expiration: option_env!("VERIFICATION_EXPIRATION")
                .unwrap_or((60 * 5).to_string().as_str())
                .parse()
                .expect("VERIFYCATION_EXPIRATION must be a number"),
            aws: AwsConfig::default(),
            from_email: option_env!("FROM_EMAIL").unwrap_or("hi@biyard.co"),
            nonce_lab: NonceLabConfig {
                endpoint: option_env!("NONCE_LAB_API_ENDPOINT")
                    .expect("NONCE_LAB_API_ENDPOINT required"),
                token: option_env!("NONCE_LAB_API_TOKEN").expect("NONCE_LAB_API_TOKEN required"),
            },
            server_key: option_env!("SERVER_KEY").expect("SERVER_KEY required"),
            bucket_name: option_env!("BUCKET_NAME").expect("BUCKET_NAME required"),
            chime_bucket_name: option_env!("CHIME_BUCKET").expect("CHIME_BUCKET required"),
            presigned_url_expiration: option_env!("PRESIGNED_URL_EXPIRATION")
                .unwrap_or((60 * 5).to_string().as_str())
                .parse()
                .expect("PRESIGNED_URL_EXPIRATION must be a number"),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}

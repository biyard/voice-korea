#![allow(unused_variables)]
use crate::{
    // group::GroupV2,
    organization::Organization,
};
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::ValidationError;
use lazy_static::lazy_static;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", action = [signup(code = String), reset(code = String)], read_action = refresh, table = users)]
pub struct User {
    #[api_model(primary_key, read_action = find_by_id)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(action = [signup, login, reset, user_signup, user_login], unique, read_action = [get_user, find_by_email])]
    #[validate(email)]
    pub email: String,
    #[api_model(action = [signup, login, reset], read_action = get_user)]
    #[validate(custom(function = "validate_hex"))]
    pub password: String,
    #[api_model(action = [user_signup])]
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: Option<String>,

    #[api_model(many_to_many = organization_members, foreign_table_name = organizations, foreign_primary_key = org_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub orgs: Vec<Organization>,
    // FIXME: error returned from database: table name \"j\" specified more than once
    // #[api_model(many_to_many = group_members, foreign_table_name = groups, foreign_primary_key = group_id, foreign_reference_key = user_id)]
    // #[serde(default)]
    // pub groups: Vec<GroupV2>,
}

fn validate_hex(value: &str) -> std::result::Result<(), ValidationError> {
    let re = regex::Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    if re.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_hex"))
    }
}

fn validate_nickname(nickname: &str) -> std::result::Result<(), ValidationError> {
    lazy_static! {
        static ref NICKNAME_REGEX: regex::Regex =
            regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-_]{1,20}$").unwrap();
    }

    if !NICKNAME_REGEX.is_match(nickname) {
        return Err(ValidationError::new("Nickname must be started with alphabet or number and only allow alphabet, number, hyphen and underscore, maximum 20 characters"));
    }

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VoiceKoreaClaim {
    pub email: String,
}

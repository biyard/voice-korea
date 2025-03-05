#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
use crate::{
    // group::GroupV2,
    organization::Organization,
};
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
use lazy_static::lazy_static;
use validator::ValidationError;

#[derive(validator::Validate)]
#[api_model(base = "/auth/v1", action = [signup(code = String), reset(code = String)], read_action = refresh, table = users, iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key, read_action = find_by_id)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(action = [signup, login, reset], unique, read_action = [get_user, find_by_email])]
    #[validate(email)]
    pub email: String,
    #[api_model(action = [signup, login, reset], read_action = get_user)]
    #[validate(custom(function = "validate_hex"))]
    pub password: String,

    #[api_model(many_to_many = organization_members, foreign_table_name = organizations, foreign_primary_key = org_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub orgs: Vec<Organization>,
    // FIXME: error returned from database: table name \"j\" specified more than once
    // #[api_model(many_to_many = group_members, foreign_table_name = groups, foreign_primary_key = group_id, foreign_reference_key = user_id)]
    // #[serde(default)]
    // pub groups: Vec<GroupV2>,
}

#[derive(validator::Validate)]
#[api_model(base = "/users/v1", read_action = user_info, table = participant_users, iter_type=QueryResponse)]
pub struct ParticipantUser {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = signup)]
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: String,
    #[api_model(action = [signup, login], read_action = [check_email], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = signup, nullable)]
    #[validate(url)]
    pub profile_url: String,
}

#[derive(validator::Validate)]
#[api_model(base = "/auth/v1/verification", table = verifications, iter_type=QueryResponse)]
pub struct Verification {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(action = send_verification_code, read_action = get_verification_code)]
    #[validate(email)]
    pub email: String,
    #[api_model(read_action = get_verification_code)]
    pub value: String,
    pub expired_at: i64,
    pub attemp_count: i32,
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

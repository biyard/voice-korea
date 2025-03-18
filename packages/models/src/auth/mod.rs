#[allow(unused_imports)]
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

// TODO: need to feat validation process
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// #[derive(Validate)]
pub struct LoginParams {
    // #[validate(email)]
    pub email: String,
    // #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct ResetParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct SignUpParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AuthActionRequest {
    // Login(LoginParams),
    Reset(ResetParams),
    SignUp(SignUpParams),
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct EmailSendParams {
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct EmailVerifyParams {
    pub id: String,
    pub value: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum VerificationActionRequest {
    SendEmail(EmailSendParams),
    VerifyEmail(EmailVerifyParams),
}

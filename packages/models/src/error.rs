#[cfg(feature = "server")]
use by_axum::{
    aide,
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    },
};
use dioxus_translate::Translate;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ApiError {
    #[translate(
        ko = "회원가입에 실패했습니다. 다시 시도해주세요.",
        en = "Sign-up failed. Please try again."
    )]
    SignupFailed(String),
    ApiCallError(String),

    DatabaseQueryError(String),

    // User Errors
    #[translate(ko = "인증코드가 잘못되었습니다.", en = "Invalid verification code.")]
    InvalidVerificationCode,
    #[translate(
        ko = "인증코드가 만료되었습니다.",
        en = "Verification code is expired."
    )]
    VerificationCodeExpired,
    #[translate(ko = "이미 가입한 사용자입니다.", en = "User already exists.")]
    DuplicateUser,
    #[translate(ko = "사용자를 찾을 수 없습니다.", en = "User not found.")]
    NoUser,

    // OrganizationMember
    #[translate(
        ko = "조직원을 만들 수 없습니다.",
        en = "Cannot create organization member."
    )]
    CannotCreateOrganizationMember,

    InvalidAction,
    Unauthorized,

    NotFound,
    Unknown(String),
    BadRequest,

    #[translate(ko = "입력값이 잘못되었습니다.", en = "Invalid input value.")]
    ValidationError(String),

    DynamoCreateException(String),

    DynamoQueryException(String),

    DynamoUpdateException(String),

    DynamoDeleteException(String),

    InvalidCredentials(String),

    JWTGenerationFail(String),

    SESServiceError(String),

    AuthKeyNotMatch(String),

    SetExpiredTimeFailed,

    PutObjectFailed,

    ReqwestFailed(String),

    JSONSerdeError(String),

    InCompleteDraft,

    ForbiddenAccessError,

    AlreadyExists,

    InvalidPermissions, // if organization is not matched with organization_member or group_member

    OrganizationNotFound,

    ResourceNotFound,

    InvalidType,

    // Survey Errors
    SurveyAlreadyExists,
    SurveyNotFound(String),
    SurveyNotDraft,

    // Survey Response Errors
    SurveyResponseMissingAnswer,
    SurveyResponseInconsistentAnswerType,
    SurveyResponseNoMatchedAttributeGroup,
    SurveyResponseNoMatchedPanelId,
    SurveyResponsePanelQuotaExceeded,
    SurveyResponseExcelWritingError,
    SurveyResponseExcelUploadError,
    SurveyResponseExcelPresigningError,
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::ApiCallError(e.to_string())
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(e: validator::ValidationErrors) -> Self {
        ApiError::ValidationError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::DatabaseQueryError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::DynamoCreateException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoQueryException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoUpdateException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoDeleteException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            ApiError::JWTGenerationFail(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SESServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthKeyNotMatch(_) => StatusCode::NOT_ACCEPTABLE,
            ApiError::DuplicateUser => StatusCode::CONFLICT,
            ApiError::ReqwestFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::JSONSerdeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SurveyNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::SurveyNotDraft => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InCompleteDraft => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::ForbiddenAccessError => StatusCode::FORBIDDEN,
            ApiError::AlreadyExists => StatusCode::ALREADY_REPORTED,
            ApiError::InvalidPermissions => StatusCode::FORBIDDEN,
            ApiError::OrganizationNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::PutObjectFailed => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ResourceNotFound => StatusCode::NOT_FOUND,
            ApiError::SetExpiredTimeFailed => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        let body = Json(self);

        (status_code, body).into_response()
    }
}

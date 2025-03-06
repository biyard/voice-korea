use dioxus_translate::Translate;

#[cfg(feature = "server")]
use by_axum::{
    aide,
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    },
};

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

    InvalidVerificationCode,
    InvalidAction,
    Unauthorized,

    NotFound,
    Unknown(String),
    BadRequest,

    ValidationError(String),

    DynamoCreateException(String),

    DynamoQueryException(String),

    DynamoUpdateException(String),

    DynamoDeleteException(String),

    InvalidCredentials(String),

    JWTGenerationFail(String),

    SESServiceError(String),

    AuthKeyNotMatch(String),

    DuplicateUser,

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

// impl std::fmt::Display for ApiError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl std::str::FromStr for ApiError {
//     type Err = String;

//     fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
//         Ok(ApiError::ApiCallError(s.to_string()))
//     }
// }

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

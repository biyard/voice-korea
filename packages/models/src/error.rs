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

    UpdateNotAllowed,

    InvalidPermissions, // if organization is not matched with organization_member or group_member

    OrganizationNotFound,

    InvalidType,

    // Resource Errors
    #[translate(
        ko = "참고자료 또는 파일을 찾을 수 없습니다.",
        en = "Cannot find the reference or file."
    )]
    ResourceNotFound,

    #[translate(
        ko = "자료에 접근권한이 없습니다.",
        en = "No access permission to the resource."
    )]
    ResourceNotPermitted,

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

    // Deliberation Errors
    #[translate(
        ko = "중복된 공론이거나 분야를 확인해주세요.",
        en = "Please check your deliberation or field."
    )]
    DeliberationException,
    #[translate(
        ko = "역할 설정을 확인해주세요.",
        en = "Please check your role settings."
    )]
    DeliberationUserException,
    #[translate(
        ko = "단계설정을 확인해주세요.",
        en = "Please check your step settings."
    )]
    DeliberationStepException,

    #[translate(
        ko = "첨부자료를 찾을 수 없습니다.",
        en = "Cannot find the attached file."
    )]
    DeliberationResourceException,
    #[translate(
        ko = "연결된 여론조사를 찾을 수 없습니다.",
        en = "Cannot find the connected survey."
    )]
    DeliberationSurveyException,
    #[translate(
        ko = "토론생성에 실패했습니다. 잠시후 다시 시도해주세요.",
        en = "Failed to create deliberation. Please try again later."
    )]
    DeliberationDiscussionException,

    #[translate(
        ko = "설정된 패널정보를 확인해주세요.",
        en = "Please check the set panel information."
    )]
    DeliberationPanelException,

    #[translate(
        ko = "선택된 공론이 정상인지 확인해주세요.",
        en = "Please check if the selected deliberation is normal."
    )]
    DeliberationNotFound,

    #[translate(
        ko = "댓글과 사용자 정보가 정확한지 확인해주세요.",
        en = "Please check the comment and user information."
    )]
    DeliberationCommentLikeException,

    #[translate(
        ko = "댓글을 가져오는데 실패했습니다. 새로고침 후 다시 시도해주세요.",
        en = "Failed to get comments. Please refresh and try again."
    )]
    DeliberationCommentException,

    #[translate(ko = "댓글을 찾을 수 없습니다.", en = "Cannot find the comment.")]
    DeliberationCommentNotFound,

    // Discussion Errors
    #[translate(
        ko = "토론에 첨부된 자료를 확인해주세요.",
        en = "Please check the attached file in the discussion."
    )]
    DiscussionResourceException,

    #[translate(
        ko = "토론에 참여한 사용자를 확인해주세요.",
        en = "Please check the users who participated in the discussion."
    )]
    DiscussionUserException,

    #[translate(ko = "토론을 찾을 수 없습니다.", en = "Cannot find the discussion.")]
    DiscussionNotFound,

    AwsChimeError(String),
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

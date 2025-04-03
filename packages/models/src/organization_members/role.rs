#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;

#[derive(Debug, Clone, PartialEq, Eq, ApiModel, Default, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Role {
    #[translate(ko = "관리자", en = "Admin")]
    Admin = 0,
    #[translate(ko = "공론 관리자", en = "Deliberation Admin")]
    DeliberationAdmin = 1,
    #[translate(ko = "분석가", en = "Analyst")]
    Analyst = 2,
    #[translate(ko = "중재자", en = "Mediator")]
    Moderator = 3,
    #[translate(ko = "강연자", en = "Speaker")]
    Speaker = 4,
    #[default]
    None = 5,
}

#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum StepType {
    #[default]
    None = 0,
    #[translate(ko = "일반 게시글", en = "General Post")]
    GeneralPost = 1,
    #[translate(ko = "화상 회의", en = "Video Conference")]
    VideoConference = 2,
    #[translate(ko = "포스트형 게시글", en = "Post")]
    Post = 3,
    #[translate(ko = "투표", en = "Vote")]
    Vote = 4,
    #[translate(ko = "보고서", en = "Report")]
    Report = 5,
}

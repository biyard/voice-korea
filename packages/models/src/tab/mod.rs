#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;
#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Tab {
    #[default]
    #[translate(ko = "기본정보", en = "Basic Info")]
    BasicInfo = 0,
    #[translate(ko = "표본 조사", en = "Sample Survey")]
    SampleSurvey = 1,
    #[translate(ko = "숙의", en = "Deliberation")]
    Deliberation = 2,
    #[translate(ko = "토론", en = "Discussion")]
    Discussion = 3,
    #[translate(ko = "투표", en = "Final Survey")]
    FinalSurvey = 4,
    #[translate(ko = "최종 권고안", en = "Final Draft")]
    FinalDraft = 5,
}

impl Tab {
    pub fn all() -> Vec<Tab> {
        vec![
            Tab::BasicInfo,
            Tab::SampleSurvey,
            Tab::Deliberation,
            Tab::Discussion,
            Tab::FinalSurvey,
            Tab::FinalDraft,
        ]
    }
}

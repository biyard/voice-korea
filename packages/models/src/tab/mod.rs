#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;
#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Tab {
    #[default]
    #[translate(ko = "기본정보", en = "Details")]
    Details = 0,
    #[translate(ko = "표본 조사", en = "SampleSurvey")]
    SampleSurvey = 1,
    #[translate(ko = "숙의", en = "Deliberation")]
    Deliberation = 2,
    #[translate(ko = "토론", en = "Discussion")]
    Discussion = 3,
    #[translate(ko = "투표", en = "Vote")]
    Vote = 4,
    #[translate(ko = "최종 권고안", en = "FinalRecommendation")]
    FinalRecommendation = 5,
}

impl Tab {
    pub fn all() -> Vec<Tab> {
        vec![
            Tab::Details,
            Tab::SampleSurvey,
            Tab::Deliberation,
            Tab::Discussion,
            Tab::Vote,
            Tab::FinalRecommendation,
        ]
    }
}

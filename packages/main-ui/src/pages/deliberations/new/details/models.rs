use bdk::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy, Translate)]
pub enum DeliberationDetailSettingStep {
    #[translate(ko = "기본 정보")]
    BasicInfo,
    #[translate(ko = "표본 조사")]
    SampleSurvey,
    #[translate(ko = "숙의")]
    Deliberation,
    #[translate(ko = "토론")]
    Discussion,
    #[translate(ko = "투표")]
    Vote,
    #[translate(ko = "최종 권고안")]
    Recommendation,
}

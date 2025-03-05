#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum ProjectArea {
    #[default]
    #[translate(ko = "경제")]
    Economy = 1,
    #[translate(ko = "사회")]
    Society = 2,
    #[translate(ko = "환경")]
    Environment = 3,
    #[translate(ko = "교육")]
    Education = 4,
    #[translate(ko = "문화")]
    Culture = 5,
    #[translate(ko = "노동")]
    Labor = 6,
    #[translate(ko = "도시")]
    City = 7,
    #[translate(ko = "기술")]
    Technology = 8,
    #[translate(ko = "보건")]
    Health = 9,
    #[translate(ko = "정치")]
    Politics = 10,
}

impl ProjectArea {
    pub fn all() -> Vec<ProjectArea> {
        vec![
            ProjectArea::Economy,
            ProjectArea::Society,
            ProjectArea::Environment,
            ProjectArea::Education,
            ProjectArea::Culture,
            ProjectArea::Labor,
            ProjectArea::City,
            ProjectArea::Technology,
            ProjectArea::Health,
            ProjectArea::Politics,
        ]
    }
}

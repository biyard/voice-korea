#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectType {
    #[default]
    #[translate(ko = "설문조사")]
    Survey = 1,
    #[translate(ko = "공론조사")]
    Deliberation = 2,
}

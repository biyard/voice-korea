use bdk::prelude::*;

// FIXME: rename to ProjectStatus after finishing migration from public_opinion.
#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectStatus {
    #[default]
    #[translate(ko = "준비")]
    Ready = 1,
    #[translate(ko = "진행")]
    InProgress = 2,
    #[translate(ko = "마감")]
    Finish = 3,
}

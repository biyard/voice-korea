pub mod profile;
pub mod v2;

use crate::{projects::ProjectArea, ProjectStatus};
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct ScheduleInfo {
    pub title: String,
    pub schedules: Vec<ScheduleDetailInfo>,
    pub typed_schedule: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct ScheduleDetailInfo {
    pub start_date: u64,
    pub end_date: u64,
    pub contents: String,
    pub options: Option<ScheduleOption>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct ScheduleOption {
    pub title: String,
    pub contents: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MeetingType {
    #[default]
    Offline,
    Online,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct DiscussionGroupInfo {
    pub total_groups: u64,
    pub groups: Vec<DiscussionGroupDetailInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct DiscussionGroupDetailInfo {
    pub name: String,
    pub discussion_count: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UpsertPanelInfo {
    pub totals: u64,
    pub allocation_method: AllocationMethod,
    pub panels: Vec<PanelAttribute>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PanelAttribute {
    pub panel: CompositionPanelInfo,
    pub panel_count: u64,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CompositionPanelInfo {
    pub panel_id: Option<String>, //panel id가 none일 경우 id 생성 후 패널 및 속성 관리 페이지 추가
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AttributeInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AllocationMethod {
    #[default]
    FairAllocated,
    ProportionalAllocation,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Document {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub volume: Option<String>, //etc. 3.5 MB
    pub projects: Option<ProjectInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum OpinionDraftStatus {
    #[default]
    Init,
    PublicOpinionComposition,
    InputInformation,
    CommitteeComposition,
    PanelComposition,
    DiscussionSetting,
    Finish,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OpinionResponse {
    pub project_id: String,
    pub opinion_type: ProjectArea,
    pub project_name: String,
    pub total_response_count: u64,
    pub response_count: u64,
    pub panels: Vec<PanelInfo>,
    pub start_date: u64,
    pub end_date: u64,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PanelInfo {
    pub id: String,
    pub name: String,
}

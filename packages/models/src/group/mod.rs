#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod member;
pub use member::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupProject {
    pub project_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupResponse {
    pub id: i64,
    pub creator: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub members: Vec<GroupMemberResponse>,
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreateGroupMember {
    pub member_name: String,
    pub member_email: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreateGroupRequest {
    pub name: String,
    pub members: Vec<CreateGroupMember>,            //그룹 내 팀원
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupInfo {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TeamMemberRequest {
    pub email: String,
    pub name: Option<String>,
    pub group: Option<GroupInfo>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum GroupActionRequest {
    Create(CreateGroupRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum GroupByIdActionRequest {
    UpdateName(String),
    Delete,
    AddTeamMember(TeamMemberRequest),
    RemoveTeamMember(String),
}

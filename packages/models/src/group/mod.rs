#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod member;
mod v2;
pub use member::*;
pub use v2::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupProject {
    pub project_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupResponse {
    pub id: String,
    pub creator: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub members: Vec<GroupMemberResponse>,
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Group {
    pub id: String,
    pub r#type: String,
    pub gsi1: String, // group#{user_id}
    pub creator: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
    pub organization_id: String,
}

impl Group {
    pub fn new(user_id: String, organization_id: String, name: String) -> Self {
        let now: i64 = chrono::Utc::now().timestamp_millis();
        Group {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: Group::get_type(),
            gsi1: Group::get_gsi1(&user_id),
            creator: user_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name,
            public_opinion_projects: vec![],
            investigation_projects: vec![],
            organization_id,
        }
    }

    pub fn get_gsi1(user_id: &str) -> String {
        format!("{}#{}", Self::get_type(), user_id)
    }

    pub fn get_gsi_deleted(user_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), user_id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#group".to_string()
    }

    pub fn get_type() -> String {
        "group".to_string()
    }
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

impl Into<Group> for (CreateGroupRequest, String, String, String) {
    fn into(self) -> Group {
        let (req, id, user_id, org_id) = self;
        let now = chrono::Utc::now().timestamp_millis();

        Group {
            id,
            r#type: Group::get_type(),
            gsi1: Group::get_gsi1(&user_id),
            creator: user_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: req.name,
            public_opinion_projects: req.public_opinion_projects,
            investigation_projects: req.investigation_projects,
            organization_id: org_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupInfo {
    pub id: String,
    pub name: String,
}

impl TryFrom<Group> for GroupInfo {
    type Error = std::fmt::Error;

    fn try_from(group: Group) -> Result<Self, Self::Error> {
        Ok(Self {
            id: group.id,
            name: group.name,
        })
    }
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

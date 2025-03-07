#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupMember {
    pub id: String,
    pub r#type: String,
    pub gsi1: String, // group#member#{group_id}
    pub gsi2: String, // group#member#{group_id}#{org_member_id}
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub group_id: String,
    pub org_member_id: String,
}

impl GroupMember {
    pub fn new(group_id: String, org_member_id: String) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: GroupMember::get_type(),
            gsi1: GroupMember::get_gsi1(&group_id),
            gsi2: GroupMember::get_gsi2(&group_id, &org_member_id),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            group_id,
            org_member_id,
        }
    }

    pub fn get_gsi1(group_id: &str) -> String {
        format!("{}#{}", Self::get_type(), group_id)
    }

    pub fn get_gsi2(group_id: &str, user_id: &str) -> String {
        format!("{}#{}#{}", Self::get_type(), group_id, user_id)
    }

    pub fn get_gsi1_deleted(group_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), group_id)
    }

    pub fn get_gsi2_deleted(group_id: &str, user_id: &str) -> String {
        format!("{}#{}#{}", Self::get_deleted_type(), group_id, user_id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#group#member".to_string()
    }

    pub fn get_type() -> String {
        "group#member".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MemberInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GroupMemberResponse {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub group_id: String,
    pub org_member_id: String,
    pub user_name: String,
    pub user_email: String,
    pub role_name: Option<String>,
    pub group_name: String,
}

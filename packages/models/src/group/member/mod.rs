#[allow(unused_imports)]
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

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

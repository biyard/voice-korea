use models::MeetingType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct MeetingInfo {
    pub meeting_type: MeetingType,
    pub title: String,
    pub description: String,
    pub start_date: i64,
    pub end_date: i64,
    pub users: i64,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaPlacementInfo {
    #[serde(rename = "AudioHostUrl")]
    pub audio_host_url: String,
    #[serde(rename = "AudioFallbackUrl")]
    pub audio_fallback_url: String,
    #[serde(rename = "ScreenDataUrl")]
    pub screen_data_url: String,
    #[serde(rename = "ScreenSharingUrl")]
    pub screen_sharing_url: String,
    #[serde(rename = "ScreenViewingUrl")]
    pub screen_viewing_url: String,
    #[serde(rename = "SignalingUrl")]
    pub signaling_url: String,
    #[serde(rename = "TurnControlUrl")]
    pub turn_control_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeetingInfo {
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
    #[serde(rename = "MediaPlacement")]
    pub media_placement: MediaPlacementInfo,
    #[serde(rename = "MediaRegion")]
    pub media_region: String,
}

pub struct AttendeeInfo {
    pub attendee_id: String,
    pub external_user_id: String,
    pub join_token: String,
}

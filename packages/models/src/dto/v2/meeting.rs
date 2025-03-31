use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip)]
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

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip)]
pub struct MeetingInfo {
    #[serde(rename = "MeetingId")]
    pub meeting_id: String,
    #[serde(rename = "MediaPlacement")]
    pub media_placement: MediaPlacementInfo,
    #[serde(rename = "MediaRegion")]
    pub media_region: String,
}

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip)]
pub struct AttendeeInfo {
    #[serde(rename = "AttendeeId")]
    pub attendee_id: String,
    #[serde(rename = "JoinToken")]
    pub join_token: String,
    #[serde(rename = "ExternalUserId")]
    pub external_user_id: String,
}

#[api_model(base = "/v2/deliberations/:deliberation-id/meeting/:discussion_id", database = skip, read_action = find_one)]
pub struct MeetingData {
    pub meeting: MeetingInfo,
    pub attendee: AttendeeInfo,
}

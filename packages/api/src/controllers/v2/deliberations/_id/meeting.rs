use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json,
    },
};
use models::{
    discussion_participants::DiscussionParticipant,
    discussions::Discussion,
    dto::{
        AttendeeInfo, MediaPlacementInfo, MeetingData, MeetingDataGetResponse, MeetingDataParam,
        MeetingDataReadActionType, MeetingInfo,
    },
    *,
};

use crate::utils::app_claims::AppClaims;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DiscussionPath {
    pub deliberation_id: i64,
    pub discussion_id: i64,
}

#[derive(Clone, Debug)]
pub struct MeetingController {
    #[allow(dead_code)]
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl MeetingController {
    async fn query(
        &self,
        _deliberation_id: i64,
        auth: Option<Authorization>,
        discussion_id: i64,
    ) -> Result<MeetingData> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };

        let discussion = Discussion::query_builder()
            .id_equals(discussion_id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(ApiError::DiscussionNotFound)?;

        let meeting_id = discussion.meeting_id.unwrap_or_default();

        let participant = DiscussionParticipant::query_builder()
            .discussion_id_equals(discussion.id)
            .user_id_equals(user_id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_optional(&self.pool)
            .await?;

        let attendee_id = participant.unwrap_or_default().participant_id;

        let meeting = client.get_meeting_info(&meeting_id).await?;
        let attendee = client.get_attendee_info(&meeting_id, &attendee_id).await?;

        let mp = meeting.media_placement().ok_or(ApiError::AwsChimeError(
            "Missing media_placement".to_string(),
        ))?;

        let meeting = MeetingInfo {
            meeting_id,
            media_region: meeting.media_region.clone().unwrap_or_default(),
            media_placement: MediaPlacementInfo {
                audio_host_url: mp.audio_host_url().unwrap_or_default().to_string(),
                audio_fallback_url: mp.audio_fallback_url().unwrap_or_default().to_string(),
                screen_data_url: mp.screen_data_url().unwrap_or_default().to_string(),
                screen_sharing_url: mp.screen_sharing_url().unwrap_or_default().to_string(),
                screen_viewing_url: mp.screen_viewing_url().unwrap_or_default().to_string(),
                signaling_url: mp.signaling_url().unwrap_or_default().to_string(),
                turn_control_url: mp.turn_control_url().unwrap_or_default().to_string(),
            },
        };

        let attendee = AttendeeInfo {
            attendee_id,
            join_token: attendee.join_token.unwrap_or_default(),
            external_user_id: attendee.external_user_id.unwrap_or_default(),
        };

        Ok(MeetingData { meeting, attendee })
    }
}

impl MeetingController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:discussion-id", get(Self::get_meeting_by_id))
            .with_state(self.clone()))
    }

    pub async fn get_meeting_by_id(
        State(ctrl): State<MeetingController>,
        Path(DiscussionPath {
            deliberation_id,
            discussion_id,
        }): Path<DiscussionPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<MeetingDataParam>,
    ) -> Result<Json<MeetingDataGetResponse>> {
        tracing::debug!("get_meeting_by_id {:?}", q);

        match q {
            MeetingDataParam::Read(param)
                if param.action == Some(MeetingDataReadActionType::FindOne) =>
            {
                Ok(Json(MeetingDataGetResponse::Read(
                    ctrl.query(deliberation_id, auth, discussion_id).await?,
                )))
            }
            _ => Err(ApiError::InvalidAction),
        }
    }
}

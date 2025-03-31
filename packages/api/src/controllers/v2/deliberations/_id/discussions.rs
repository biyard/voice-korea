use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use deliberation::Deliberation;
use discussion_resources::DiscussionResource;
use discussions::*;
use models::{
    discussion_participants::DiscussionParticipant,
    dto::{MediaPlacementInfo, MeetingInfo},
    *,
};
use sqlx::{postgres::PgRow, Postgres, Transaction};

use crate::utils::app_claims::AppClaims;

#[derive(Clone, Debug)]
pub struct DiscussionController {
    repo: DiscussionRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DiscussionController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DiscussionQuery,
    ) -> Result<QueryResponse<DiscussionSummary>> {
        let mut total_count = 0;
        let items: Vec<DiscussionSummary> = DiscussionSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .deliberation_id_equals(deliberation_id)
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn participant_meeting(
        &self,
        id: i64,
        auth: Option<Authorization>,
    ) -> Result<Discussion> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
        let pr = DiscussionParticipant::get_repository(self.pool.clone());

        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };

        if user_id == 0 {
            return Err(ApiError::NoUser);
        }

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(ApiError::DiscussionNotFound)?;

        if discussion.meeting_id.is_none() {
            return Err(ApiError::AwsChimeError("Not Found Meeting ID".to_string()));
        }

        let participant = DiscussionParticipant::query_builder()
            .discussion_id_equals(discussion.id)
            .user_id_equals(user_id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_optional(&self.pool)
            .await?;

        if participant.is_some() {
            return Ok(discussion);
        }

        let meeting_id = discussion.meeting_id.unwrap();
        let meeting = client.get_meeting_info(&meeting_id).await?;

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

        // NOTE: if not found participant, create participants in discussion.
        let participant = match client
            .create_attendee(&meeting, user_id.to_string().as_str())
            .await
        {
            Ok(rst) => rst,
            Err(e) => {
                tracing::error!("create attendee {}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        match pr.insert(id, user_id, participant.attendee_id).await {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("insert db failed after create participant {}", e);
                return Err(ApiError::CreateUserFailed(e.to_string()));
            }
        };

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(ApiError::DiscussionNotFound)?;

        Ok(discussion)
    }

    // TODO(api): if you want start (activate) meeting, you should using amazon-chime-sdk-js in client side.
    //       this code is just for create meeting room and get meeting id not for online link.
    async fn start_meeting(&self, id: i64, _auth: Option<Authorization>) -> Result<Discussion> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(ApiError::DiscussionNotFound)?;

        if discussion.meeting_id.is_some() {
            return Ok(discussion);
        }

        let name = discussion.name;

        let meeting = match client.create_meeting(&name).await {
            Ok(rst) => rst,
            Err(e) => {
                tracing::error!("start_meeting {}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        let discussion = match self
            .repo
            .update(
                id,
                DiscussionRepositoryUpdateRequest {
                    deliberation_id: None,
                    started_at: None,
                    ended_at: None,
                    name: None,
                    description: None,
                    meeting_id: Some(meeting.meeting_id.unwrap_or_default()),
                },
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("start_meeting {}", e);
                return Err(ApiError::DiscussionNotFound);
            }
        };

        Ok(discussion)
    }

    async fn create(
        &self,
        deliberation_id: i64,
        auth: Option<Authorization>,
        DiscussionCreateRequest {
            started_at,
            ended_at,
            name,
            description,
            resources,
        }: DiscussionCreateRequest,
    ) -> Result<Discussion> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let repo = DiscussionResource::get_repository(self.pool.clone());

        let mut tx = self.pool.begin().await?;

        let org_id = self
            .verify_permission(&mut tx, deliberation_id, user_id)
            .await?;

        let res = self
            .repo
            .insert_with_tx(
                &mut *tx,
                deliberation_id,
                started_at,
                ended_at,
                name,
                description,
                None,
            )
            .await?
            .ok_or(ApiError::DeliberationNotFound)?;

        for resource_id in resources {
            let rsc = ResourceFile::query_builder()
                .id_equals(resource_id)
                .query()
                .map(ResourceFile::from)
                .fetch_optional(&mut *tx)
                .await?
                .ok_or(ApiError::ResourceNotFound)?;

            if rsc.org_id != org_id {
                tracing::error!("It seems to try abusing system: user_id: {user_id}. It used invalid resource: {resource_id} {org_id}");
                return Err(ApiError::ResourceNotPermitted);
            }

            repo.insert_with_tx(&mut *tx, res.id, resource_id)
                .await?
                .ok_or(ApiError::ResourceNotFound)?;
        }

        let res = match Discussion::query_builder()
            .id_equals(res.id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&mut *tx)
            .await?
        {
            Some(v) => v,
            None => return Err(ApiError::DiscussionNotFound),
        };

        tx.commit().await?;

        Ok(res)
    }

    async fn update(
        &self,
        deliberation_id: i64,
        id: i64,
        auth: Option<Authorization>,
        param: DiscussionUpdateRequest,
    ) -> Result<Discussion> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let mut tx = self.pool.begin().await?;

        let _org_id = self
            .verify_permission(&mut tx, deliberation_id, user_id)
            .await?;

        let res = match self.repo.update_with_tx(&mut *tx, id, param.into()).await? {
            Some(v) => v,
            None => return Err(ApiError::DiscussionNotFound),
        };

        tx.commit().await?;

        Ok(res)
    }

    async fn delete(
        &self,
        deliberation_id: i64,
        id: i64,
        auth: Option<Authorization>,
    ) -> Result<Discussion> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let mut tx = self.pool.begin().await?;

        let _org_id = self
            .verify_permission(&mut tx, deliberation_id, user_id)
            .await?;

        let res = self
            .repo
            .delete_with_tx(&mut *tx, id)
            .await?
            .ok_or(ApiError::DiscussionNotFound)?;

        tx.commit().await?;

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     DiscussionReadAction { action, .. }: DiscussionReadAction,
    // ) -> Result<Discussion> {
    //     todo!()
    // }
}

impl DiscussionController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Discussion::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_discussion_by_id).post(Self::act_discussion_by_id),
            )
            .with_state(self.clone())
            .route("/", post(Self::act_discussion).get(Self::get_discussion))
            .with_state(self.clone())
    }

    pub async fn act_discussion(
        State(ctrl): State<DiscussionController>,
        Path(DiscussionParentPath { deliberation_id }): Path<DiscussionParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<DiscussionAction>,
    ) -> Result<Json<Discussion>> {
        tracing::debug!("act_discussion {} {:?}", deliberation_id, body);
        match body {
            DiscussionAction::Create(param) => {
                let res = ctrl.create(deliberation_id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_discussion_by_id(
        State(ctrl): State<DiscussionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(DiscussionPath {
            deliberation_id,
            id,
        }): Path<DiscussionPath>,
        Json(body): Json<DiscussionByIdAction>,
    ) -> Result<Json<Discussion>> {
        tracing::debug!(
            "act_discussion_by_id {} {:?} {:?}",
            deliberation_id,
            id,
            body
        );

        match body {
            DiscussionByIdAction::Update(param) => {
                let res = ctrl.update(deliberation_id, id, auth, param).await?;
                Ok(Json(res))
            }
            DiscussionByIdAction::Delete(_) => {
                let res = ctrl.delete(deliberation_id, id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::StartMeeting(_) => {
                let res = ctrl.start_meeting(id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::ParticipantMeeting(_) => {
                let res = ctrl.participant_meeting(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_discussion_by_id(
        State(ctrl): State<DiscussionController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DiscussionPath {
            deliberation_id,
            id,
        }): Path<DiscussionPath>,
    ) -> Result<Json<Discussion>> {
        tracing::debug!("get_discussion {} {:?}", deliberation_id, id);
        Ok(Json(
            Discussion::query_builder()
                .id_equals(id)
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(Discussion::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_discussion(
        State(ctrl): State<DiscussionController>,
        Path(DiscussionParentPath { deliberation_id }): Path<DiscussionParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DiscussionParam>,
    ) -> Result<Json<DiscussionGetResponse>> {
        tracing::debug!("list_discussion {} {:?}", deliberation_id, q);

        match q {
            DiscussionParam::Query(param) => Ok(Json(DiscussionGetResponse::Query(
                ctrl.query(deliberation_id, auth, param).await?,
            ))),
            // DiscussionParam::Read(param)
            //     if param.action == Some(DiscussionReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(DiscussionGetResponse::Read(res)))
            // }
        }
    }

    pub async fn verify_permission(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        deliberation_id: i64,
        user_id: i64,
    ) -> Result<i64> {
        let deliberation = Deliberation::query_builder()
            .id_equals(deliberation_id)
            .query()
            .map(Deliberation::from)
            .fetch_optional(&mut **tx)
            .await?
            .ok_or(ApiError::DeliberationNotFound)?;

        let user = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(User::from)
            .fetch_optional(&mut **tx)
            .await?
            .ok_or(ApiError::NoUser)?;

        user.orgs
            .iter()
            .find(|org| org.id == deliberation.org_id)
            .ok_or(ApiError::NoUser)?;

        Ok(deliberation.org_id)
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DiscussionPath {
    pub deliberation_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DiscussionParentPath {
    pub deliberation_id: i64,
}

#[cfg(test)]
mod discussion_tests {
    use crate::tests::{setup, TestContext};

    use super::*;

    async fn create_deliberation(endpoint: &str, org_id: i64, now: i64) -> i64 {
        let get_client = Deliberation::get_client(endpoint);
        let cli = get_client;
        let res = cli
            .create(
                org_id,
                now,
                now + 1000,
                ProjectArea::City,
                format!("test deliberation {now}"),
                "test description".to_string(),
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            )
            .await;
        assert!(res.is_ok());

        res.unwrap().id
    }

    #[tokio::test]
    async fn test_create_discussion() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let org_id = user.orgs[0].id;

        let deliberation_id = create_deliberation(&endpoint, org_id, now).await;

        let cli = Discussion::get_client(&endpoint);
        let started_at = now;
        let ended_at = now + 10000;
        let name = "test discussion".to_string();
        let description = "test description".to_string();

        let discussion = cli
            .create(
                deliberation_id,
                started_at,
                ended_at,
                name.clone(),
                description.clone(),
                vec![],
            )
            .await
            .unwrap();

        assert_eq!(discussion.deliberation_id, deliberation_id);
        assert_eq!(discussion.started_at, started_at);
        assert_eq!(discussion.ended_at, ended_at);
        assert_eq!(discussion.name, name);
        assert_eq!(discussion.description, description);

        let got = Discussion::get_client(&endpoint)
            .get(deliberation_id, discussion.id)
            .await
            .unwrap();

        assert_eq!(got.id, discussion.id);
        assert_eq!(got.deliberation_id, deliberation_id);
        assert_eq!(got.started_at, started_at);
        assert_eq!(got.ended_at, ended_at);
        assert_eq!(got.name, name);
        assert_eq!(got.description, description);
    }
}

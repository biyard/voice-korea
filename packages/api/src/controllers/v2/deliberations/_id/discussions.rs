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
use models::*;
use sqlx::postgres::PgRow;

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

    async fn start_meeting(&self, _id: i64, _auth: Option<Authorization>) -> Result<Discussion> {
        todo!()
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

        let deliberation = Deliberation::query_builder()
            .id_equals(deliberation_id)
            .query()
            .map(Deliberation::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(ApiError::DeliberationNotFound)?;

        let user = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(User::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(ApiError::NoUser)?;

        user.orgs
            .iter()
            .find(|org| org.id == deliberation.org_id)
            .ok_or(ApiError::NoUser)?;

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

        let org_id = deliberation.org_id;

        for resource_id in resources {
            let rsc = ResourceFile::query_builder()
                .id_equals(resource_id)
                .query()
                .map(ResourceFile::from)
                .fetch_optional(&self.pool)
                .await?
                .ok_or(ApiError::ResourceNotFound)?;

            if rsc.org_id != org_id {
                tracing::error!("It seems to try abusing system: {auth:?}. It used invalid resource: {resource_id} {org_id}");
                return Err(ApiError::ResourceNotPermitted)?;
            }

            repo.insert_with_tx(&mut *tx, res.id, resource_id)
                .await?
                .ok_or(ApiError::ResourceNotFound)?;
        }

        let res = Discussion::query_builder()
            .id_equals(res.id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(ApiError::DiscussionNotFound)?;

        tx.commit().await?;

        Ok(res)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: DiscussionUpdateRequest,
    ) -> Result<Discussion> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Discussion> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

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
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            DiscussionByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::StartMeeting(_) => {
                let res = ctrl.start_meeting(id, auth).await?;
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

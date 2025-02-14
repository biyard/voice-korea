use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::*;

#[derive(Clone, Debug)]
pub struct GroupControllerV2 {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: GroupV2Repository,
    group_mem: GroupMemberV2Repository,
    // user: UserRepository,
}

impl GroupControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = GroupV2::get_repository(pool.clone());
        // let user = User::get_repository(pool.clone());
        let group_mem = GroupMemberV2::get_repository(pool.clone());
        let ctrl = GroupControllerV2 {
            pool,
            repo,
            group_mem,
        };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_group).post(Self::act_group_by_id))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_group).get(Self::list_group))
            .with_state(ctrl.clone()))
    }

    pub async fn act_group(
        State(ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Json(body): Json<GroupV2Action>,
    ) -> Result<Json<GroupV2>> {
        tracing::debug!("act_group {:?}", body);

        match body {
            GroupV2Action::Create(req) => ctrl.create_group(org_id, req).await,
            GroupV2Action::Delete(req) => ctrl.delete_group(req).await,
        }
    }

    pub async fn act_group_by_id(
        State(_ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
        Json(body): Json<GroupV2ByIdAction>,
    ) -> Result<Json<GroupV2>> {
        tracing::debug!("act_group_by_id {:?} {:?}", id, body);
        Ok(Json(GroupV2::default()))
    }

    pub async fn get_group(
        State(ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<GroupV2>> {
        tracing::debug!("get_group {:?}", id);

        ctrl.find_group_by_id(id).await
    }

    pub async fn list_group(
        State(ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Query(param): Query<GroupV2Param>,
    ) -> Result<Json<GroupV2GetResponse>> {
        tracing::debug!("list_group {:?}", param);
        match param {
            GroupV2Param::Query(q) => {
                let groups = ctrl.list_group_by_id(org_id, q).await?;
                Ok(Json(groups))
            }
            _ => Err(ApiError::InvalidAction),
        }
    }
}

impl GroupControllerV2 {
    async fn create_group(&self, org_id: i64, req: GroupV2CreateRequest) -> Result<Json<GroupV2>> {
        let group = self.repo.insert(org_id, req.name).await?;

        for user in req.users {
            self.group_mem.insert(group.id, user.id).await?;
        }

        Ok(Json(group))
    }

    async fn delete_group(&self, req: GroupV2DeleteRequest) -> Result<Json<GroupV2>> {
        let group = self
            .repo
            .find_one(&GroupV2ReadAction::new().find_by_id(req.id))
            .await?;
        self.repo.delete(req.id).await?;
        Ok(Json(group))
    }

    async fn find_group_by_id(&self, id: i64) -> Result<Json<GroupV2>> {
        let group = self
            .repo
            .find_one(&GroupV2ReadAction::new().find_by_id(id))
            .await?;
        Ok(Json(group))
    }

    async fn list_group_by_id(
        &self,
        org_id: i64,
        q: GroupV2Query,
    ) -> Result<Json<GroupV2GetResponse>> {
        let query = GroupV2Summary::base_sql_with("where org_id = $1 limit $2 offset $3");
        tracing::debug!("list_group_by_id query: {:?}", query);

        let mut total_count: i64 = 0;
        let items = sqlx::query(&query)
            .bind(org_id)
            .bind(q.size as i64)
            .bind(
                q.size as i64
                    * (q.bookmark
                        .unwrap_or("1".to_string())
                        .parse::<i64>()
                        .unwrap()
                        - 1),
            )
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(GroupV2GetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }
}

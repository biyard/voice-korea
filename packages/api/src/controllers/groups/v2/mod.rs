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
    user: UserRepository,
    org_mem: OrganizationMemberRepository,
}

impl GroupControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = GroupV2::get_repository(pool.clone());
        let group_mem = GroupMemberV2::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let org_mem = OrganizationMember::get_repository(pool.clone());
        let ctrl = GroupControllerV2 {
            pool,
            repo,
            group_mem,
            user,
            org_mem,
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
            GroupV2Action::Delete(req) => ctrl.delete_group(org_id, req).await,
        }
    }

    pub async fn act_group_by_id(
        State(ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
        Json(body): Json<GroupV2ByIdAction>,
    ) -> Result<Json<GroupV2>> {
        tracing::debug!("act_group_by_id {:?} {:?}", id, body);

        // TODO: need org - group relation validation

        match body {
            GroupV2ByIdAction::Update(req) => Ok(Json(ctrl.update_group(id, req).await?)),
            GroupV2ByIdAction::AddGroupMember(req) => {
                Ok(Json(ctrl.add_group_member(id, org_id, req).await?))
            }
            GroupV2ByIdAction::RemoveGroupMember(req) => {
                Ok(Json(ctrl.remove_group_member(id, org_id, req).await?))
            }
        }
    }

    pub async fn get_group(
        State(ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
    ) -> Result<Json<GroupV2>> {
        tracing::debug!("get_group {:?}", id);

        Ok(Json(ctrl.find_group_by_id(org_id, id).await?))
    }

    pub async fn list_group(
        State(ctrl): State<GroupControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Query(param): Query<GroupV2Param>,
    ) -> Result<Json<GroupV2GetResponse>> {
        tracing::debug!("list_group {:?}", param);
        match param {
            GroupV2Param::Query(q) => ctrl.list_groups_by_id(org_id, q).await,
            // _ => Err(ApiError::InvalidAction),
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

    async fn delete_group(&self, org_id: i64, req: GroupV2DeleteRequest) -> Result<Json<GroupV2>> {
        // let group = self
        //     .repo
        //     .find_one(&GroupV2ReadAction::new().find_group_by_id(req.id))
        //     .await?;
        let group = self.find_group_by_id(org_id, req.id).await?;
        self.repo.delete(req.id).await?;
        Ok(Json(group))
    }

    async fn find_group_by_id(&self, org_id: i64, id: i64) -> Result<GroupV2> {
        // let group = self
        //     .repo
        //     .find_one(&GroupV2ReadAction::new().find_group_by_id(id))
        //     .await?;

        let query = GroupV2Summary::base_sql_with("where id = $1 AND org_id = $2");
        let group: GroupV2 = sqlx::query(&query)
            .bind(id)
            .bind(org_id)
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await?;

        // let group_mem = self
        //     .group_mem
        //     .find(&GroupMemberV2QueryAction::new().query_by_group_id(id))
        //     .await?;

        // TODO: need to add user info to group

        Ok(group)
    }

    async fn list_groups_by_id(
        &self,
        org_id: i64,
        q: GroupV2Query,
    ) -> Result<Json<GroupV2GetResponse>> {
        let query = GroupV2Summary::base_sql_with("where org_id = $1 limit $2 offset $3 ");
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

    async fn update_group(&self, id: i64, req: GroupV2UpdateRequest) -> Result<GroupV2> {
        let group = self.repo.update(id, req.into()).await?;
        Ok(group)
    }

    async fn add_group_member(
        &self,
        id: i64,
        org_id: i64,
        req: GroupV2AddGroupMemberRequest,
    ) -> Result<GroupV2> {
        let group = match self.find_group_by_id(org_id, id).await {
            Ok(g) => g,
            Err(e) => {
                tracing::error!("add_group_member: {:?}", e);
                return Err(ApiError::NotFound.into());
            }
        };

        let user = match self
            .user
            .find_one(&UserReadAction::new().find_by_email(req.email))
            .await
        {
            Ok(u) => u,
            Err(e) => {
                tracing::error!("add_group_member: {:?}", e);
                return Err(ApiError::NotFound.into());
            }
        };

        match self
            .org_mem
            .find_one(&OrganizationMemberReadAction::new().get_member(user.id))
            .await
        {
            Ok(o) => {
                if o.org_id != org_id {
                    return Err(ApiError::Unauthorized.into());
                }
            }
            Err(e) => {
                tracing::error!("add_group_member: {:?}", e);
                return Err(ApiError::NotFound.into());
            }
        }

        self.group_mem.insert(id, user.id).await?;

        Ok(group)
    }

    async fn remove_group_member(
        &self,
        id: i64,
        org_id: i64,
        req: GroupV2RemoveGroupMemberRequest,
    ) -> Result<GroupV2> {
        let group = match self.find_group_by_id(org_id, id).await {
            Ok(g) => g,
            Err(e) => {
                tracing::error!("remove_group_member: {:?}", e);
                return Err(ApiError::NotFound.into());
            }
        };

        let query = GroupMemberV2Summary::base_sql_with("where group_id = $1 AND user_id = $2");
        tracing::debug!("remove_group_member query: {:?}", query);

        let item: GroupMemberV2 = match sqlx::query(&query)
            .bind(id)
            .bind(req.user_id)
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await
        {
            Ok(i) => i,
            Err(e) => {
                tracing::error!("remove_group_member: {:?}", e);
                return Err(ApiError::NotFound.into());
            }
        };

        self.group_mem.delete(item.id).await?;

        Ok(group)
    }
}

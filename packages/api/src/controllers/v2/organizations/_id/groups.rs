use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::*;

#[derive(Clone, Debug)]
pub struct GroupController {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: GroupRepository,
    user: UserRepository,
    org_mem: OrganizationMemberRepository,
}

impl GroupController {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Group::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let org_mem = OrganizationMember::get_repository(pool.clone());

        let ctrl = GroupController {
            pool,
            repo,
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
        State(ctrl): State<GroupController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(GroupParentPath { org_id }): Path<GroupParentPath>,
        Json(body): Json<GroupAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act_group {:?}", body);

        match body {
            GroupAction::Create(req) => ctrl.create_group(org_id, req).await,
        }
    }

    pub async fn act_group_by_id(
        State(ctrl): State<GroupController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(GroupPath { org_id, id }): Path<GroupPath>,
        Json(body): Json<GroupByIdAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act_group_by_id {:?} {:?}", id, body);

        let res = match body {
            GroupByIdAction::Update(req) => ctrl.update_group(id, req).await?,
            GroupByIdAction::AddGroupMember(req) => ctrl.add_group_member(id, org_id, req).await?,
            GroupByIdAction::RemoveGroupMember(req) => {
                ctrl.remove_group_member(id, org_id, req).await?
            }
            GroupByIdAction::Delete(_) => ctrl.delete_group(id).await?,
        };

        Ok(Json(res))
    }

    pub async fn get_group(
        State(ctrl): State<GroupController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(GroupPath { org_id, id }): Path<GroupPath>,
    ) -> Result<Json<Group>> {
        tracing::debug!("get_group {:?}", id);

        Ok(Json(ctrl.find_group_by_id(org_id, id).await?))
    }

    pub async fn list_group(
        State(ctrl): State<GroupController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(GroupParentPath { org_id }): Path<GroupParentPath>,
        Query(param): Query<GroupParam>,
    ) -> Result<Json<GroupGetResponse>> {
        tracing::debug!("list_group {:?}", param);
        match param {
            GroupParam::Query(q) => ctrl.list_groups_by_id(org_id, q).await,
            // _ => Err(ApiError::InvalidAction),
        }
    }
}

impl GroupController {
    async fn create_group(&self, org_id: i64, req: GroupCreateRequest) -> Result<Json<Group>> {
        let group = self.repo.insert(org_id, req.name).await?;

        Ok(Json(group))
    }

    async fn delete_group(&self, id: i64) -> Result<Group> {
        Ok(self.repo.delete(id).await?)
    }

    async fn find_group_by_id(&self, org_id: i64, id: i64) -> Result<Group> {
        // let group = self
        //     .repo
        //     .find_one(&GroupReadAction::new().find_group_by_id(id))
        //     .await?;

        let query = GroupSummary::base_sql_with("where id = $1 AND org_id = $2");
        let group: Group = sqlx::query(&query)
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
        q: GroupQuery,
    ) -> Result<Json<GroupGetResponse>> {
        let query = GroupSummary::base_sql_with("where org_id = $1 limit $2 offset $3 ");
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

        Ok(Json(GroupGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    async fn update_group(&self, id: i64, req: GroupUpdateRequest) -> Result<Group> {
        let group = self.repo.update(id, req.into()).await?;
        Ok(group)
    }

    async fn add_group_member(
        &self,
        id: i64,
        org_id: i64,
        req: GroupAddGroupMemberRequest,
    ) -> Result<Group> {
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

        Ok(group)
    }

    async fn remove_group_member(
        &self,
        _id: i64,
        _org_id: i64,
        _req: GroupRemoveGroupMemberRequest,
    ) -> Result<Group> {
        unimplemented!()
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct GroupPath {
    pub org_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct GroupParentPath {
    pub org_id: i64,
}

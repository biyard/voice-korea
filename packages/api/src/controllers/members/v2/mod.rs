use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::*;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct MemberControllerV2 {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: OrganizationMemberRepository,
    user: UserRepository,
}

impl MemberControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = OrganizationMember::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let ctrl = MemberControllerV2 { pool, repo, user };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_member).post(Self::act_member))
            .with_state(ctrl.clone())
            .route(
                "/:user_id",
                post(Self::act_member_by_id).get(Self::get_member),
            )
            .with_state(ctrl.clone()))
    }

    pub async fn act_member(
        State(ctrl): State<MemberControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Json(body): Json<OrganizationMemberAction>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!("act_member {:?}", body);

        match body {
            OrganizationMemberAction::Delete(req) => ctrl.delete_member(org_id, req.user_id).await,
        }
    }

    pub async fn act_member_by_id(
        State(ctrl): State<MemberControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, user_id)): Path<(i64, i64)>,
        Json(body): Json<OrganizationMemberByIdAction>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!("act_member_by_id {:?} {:?} {:?}", org_id, user_id, body);

        match body {
            OrganizationMemberByIdAction::Update(req) => {
                ctrl.update_member(org_id, user_id, req).await
            }
        }
    }

    pub async fn get_member(
        State(ctrl): State<MemberControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, user_id)): Path<(i64, i64)>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!("get_member {:?}", user_id);

        ctrl.get_member_by_user_id(org_id, user_id).await
    }

    pub async fn list_member(
        State(ctrl): State<MemberControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Query(param): Query<OrganizationMemberParam>,
    ) -> Result<Json<ListMemberResponseV2>> {
        tracing::debug!("list_member {:?}", param);

        match param {
            OrganizationMemberParam::Query(q) => ctrl.list_member_by_org_id(org_id, q).await,
            _ => Err(ApiError::InvalidAction),
        }
    }
}

impl MemberControllerV2 {
    async fn get_member_by_user_id(
        &self,
        org_id: i64,
        user_id: i64,
    ) -> Result<Json<OrganizationMember>> {
        let query = OrganizationMemberSummary::base_sql_with("where org_id = $1 AND user_id = $2");
        tracing::debug!("get_member query: {:?}", query);

        let member = match sqlx::query(&query)
            .bind(org_id)
            .bind(user_id)
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to find member: {}", e);
                return Err(ApiError::InvalidPermissions);
            }
        };

        Ok(Json(member))
    }

    async fn list_member_by_org_id(
        &self,
        org_id: i64,
        q: OrganizationMemberQuery,
    ) -> Result<Json<ListMemberResponseV2>> {
        let query =
            OrganizationMemberSummary::base_sql_with("where org_id = $1 limit $2 offset $3");
        tracing::debug!("list_member query: {:?}", query);

        let mut total_count: i64 = 0;
        let members: Vec<OrganizationMember> = sqlx::query(&query)
            .bind(org_id)
            .bind(q.size as i64)
            .bind(
                q.size as i64
                    * (q.bookmark
                        .clone()
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

        let mut role_count = vec![total_count, 0, 0, 0, 0, 0]; //[전체, 관리자, 공론 관리자, 분석가, 중계자, 강연자]

        for member in members.clone() {
            match member.role {
                Some(Role::Admin) => role_count[1] += 1,
                Some(Role::PublicAdmin) => role_count[2] += 1,
                Some(Role::Analyst) => role_count[3] += 1,
                Some(Role::Mediator) => role_count[4] += 1,
                Some(Role::Speaker) => role_count[5] += 1,
                _ => {}
            }
        }

        Ok(Json(ListMemberResponseV2 {
            members,
            role_count,
            bookmark: q.bookmark,
        }))
    }

    async fn update_member(
        &self,
        org_id: i64,
        user_id: i64,
        params: OrganizationMemberUpdateRequest,
    ) -> Result<Json<OrganizationMember>> {
        let query = OrganizationMemberSummary::base_sql_with("where org_id = $1 AND user_id = $2");
        tracing::debug!("update_member query: {:?}", query);

        let member: OrganizationMember = match sqlx::query(&query)
            .bind(org_id)
            .bind(user_id)
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to find member: {}", e);
                return Err(ApiError::InvalidPermissions);
            }
        };

        self.repo.update(member.id, params.into()).await?;

        Ok(Json(member))
    }

    async fn delete_member(&self, org_id: i64, user_id: i64) -> Result<Json<OrganizationMember>> {
        let query = OrganizationMemberSummary::base_sql_with("where org_id = $1 AND user_id = $2");
        tracing::debug!("delete_member query: {:?}", query);

        let member: OrganizationMember = match sqlx::query(&query)
            .bind(org_id)
            .bind(user_id)
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_one(&self.pool)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to find member: {}", e);
                return Err(ApiError::InvalidPermissions);
            }
        };

        self.repo.delete(member.id).await?;

        Ok(Json(member))
    }
}

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
use models::{deliberation::Deliberation, *};
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct OrganizationMemberController {
    repo: OrganizationMemberRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl OrganizationMemberController {
    async fn query(
        &self,
        org_id: i64,
        auth: Option<Authorization>,
        param: OrganizationMemberQuery,
    ) -> Result<QueryResponse<OrganizationMemberSummary>> {
        tracing::debug!("{param}");
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let mut total_count = 0;
        let items: Vec<OrganizationMemberSummary> = OrganizationMemberSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .org_id_equals(org_id)
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

    async fn create(
        &self,
        org_id: i64,
        auth: Option<Authorization>,
        OrganizationMemberCreateRequest {
            name,
            role,
            contact,
            email,
        }: OrganizationMemberCreateRequest,
    ) -> Result<OrganizationMember> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let mut tx = self.pool.begin().await?;

        let user = User::query_builder()
            .email_equals(email)
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ApiError::NoUser)?;

        let member = self
            .repo
            .insert_with_tx(&mut *tx, user.id, org_id, name, role, contact)
            .await?
            .ok_or(ApiError::CannotCreateOrganizationMember)?;

        tx.commit().await?;

        Ok(member)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: OrganizationMemberUpdateRequest,
    ) -> Result<OrganizationMember> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let member = self.repo.update(id, param.into()).await?;

        Ok(member)
    }

    async fn update_role(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: OrganizationMemberUpdateRoleRequest,
    ) -> Result<OrganizationMember> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let member = self.repo.update(id, param.into()).await?;

        Ok(member)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<OrganizationMember> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let member = self.repo.delete(id).await?;

        Ok(member)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     OrganizationMemberReadAction { action, .. }: OrganizationMemberReadAction,
    // ) -> Result<OrganizationMember> {
    //     todo!()
    // }
}

impl OrganizationMemberController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = OrganizationMember::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_organization_member_by_id).post(Self::act_organization_member_by_id),
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_organization_member).get(Self::get_organization_member),
            )
            .with_state(self.clone())
    }

    pub async fn act_organization_member(
        State(ctrl): State<OrganizationMemberController>,
        Path(OrganizationMemberParentPath { org_id }): Path<OrganizationMemberParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<OrganizationMemberAction>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!("act_organization_member {} {:?}", org_id, body);

        match body {
            OrganizationMemberAction::Create(param) => {
                let res = ctrl.create(org_id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_organization_member_by_id(
        State(ctrl): State<OrganizationMemberController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(OrganizationMemberPath { org_id, id }): Path<OrganizationMemberPath>,
        Json(body): Json<OrganizationMemberByIdAction>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!(
            "act_organization_member_by_id {} {:?} {:?}",
            org_id,
            id,
            body
        );

        match body {
            OrganizationMemberByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }

            OrganizationMemberByIdAction::UpdateRole(param) => {
                let res = ctrl.update_role(id, auth, param).await?;
                Ok(Json(res))
            }
            OrganizationMemberByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_organization_member_by_id(
        State(ctrl): State<OrganizationMemberController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(OrganizationMemberPath { org_id, id }): Path<OrganizationMemberPath>,
    ) -> Result<Json<OrganizationMember>> {
        tracing::debug!("get_organization_member {} {:?}", org_id, id);
        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .org_id_equals(org_id)
                .query()
                .map(OrganizationMember::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_organization_member(
        State(ctrl): State<OrganizationMemberController>,
        Path(OrganizationMemberParentPath { org_id }): Path<OrganizationMemberParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<OrganizationMemberParam>,
    ) -> Result<Json<OrganizationMemberGetResponse>> {
        tracing::debug!("list_organization_member {} {:?}", org_id, q);

        match q {
            OrganizationMemberParam::Query(param) => Ok(Json(
                OrganizationMemberGetResponse::Query(ctrl.query(org_id, auth, param).await?),
            )),
            _ => {
                unimplemented!()
            } // OrganizationMemberParam::Read(param)
              //     if param.action == Some(OrganizationMemberReadActionType::ActionType) =>
              // {
              //     let res = ctrl.run_read_action(auth, param).await?;
              //     Ok(Json(OrganizationMemberGetResponse::Read(res)))
              // }
        }
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct OrganizationMemberPath {
    pub org_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct OrganizationMemberParentPath {
    pub org_id: i64,
}

use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, State},
        routing::{get, post},
        Extension, Json,
    },
};
use models::{
    invitation::{
        Invitation, InvitationAction, InvitationGetResponse, InvitationInviteRequest,
        InvitationRepository, InvitationSummary,
    },
    *,
};

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct InvitationControllerV2 {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: InvitationRepository,
    user: UserRepository,
    member: OrganizationMemberRepository,
    // group_member: GroupMemberRepository,
}

impl InvitationControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Invitation::get_repository(pool.clone());
        let user = User::get_repository(pool.clone());
        let member = OrganizationMember::get_repository(pool.clone());
        let ctrl = InvitationControllerV2 {
            pool,
            repo,
            user,
            member,
        };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_invitation))
            .with_state(ctrl.clone())
            .route("/:user_id", get(Self::list_invitation))
            .with_state(ctrl.clone()))
    }

    pub async fn act_invitation(
        State(ctrl): State<InvitationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Json(body): Json<InvitationAction>,
    ) -> Result<Json<String>> {
        tracing::debug!("act_invitation {} {:?}", org_id, body);
        match body {
            InvitationAction::Invite(req) => Ok(Json(ctrl.invite_member(org_id, req).await?)),
        }
    }

    pub async fn list_invitation(
        State(ctrl): State<InvitationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((_org_id, user_id)): Path<(i64, i64)>,
    ) -> Result<Json<InvitationGetResponse>> {
        tracing::debug!("list_invitation {:?}", user_id);

        let user = ctrl
            .user
            .find_one(&UserReadAction::new().find_by_id(user_id))
            .await?;

        ctrl.query_by_email(user.email).await
    }
}

impl InvitationControllerV2 {
    async fn invite_member(&self, org_id: i64, req: InvitationInviteRequest) -> Result<String> {
        tracing::debug!("invite_member {} {:?}", org_id, req);
        let user = match self
            .user
            .find_one(&UserReadAction::new().find_by_email(req.email.clone()))
            .await
        {
            Ok(user) => user,
            Err(_) => {
                self.repo
                    .insert(org_id, req.group_id, req.email.clone(), req.name, req.role)
                    .await?;
                return Ok(format!("successfully invite {}", req.email));
            }
        };

        let member = self
            .member
            .insert(
                user.id,
                org_id,
                req.name.unwrap_or_else(|| user.email.to_string()),
                req.role,
                None,
            )
            .await?;

        let msg = format!("member created successfully: {}", member.id);

        // TODO: invite group
        // let group_members = self
        //     .gr
        Ok(msg)
    }

    async fn query_by_email(&self, email: String) -> Result<Json<InvitationGetResponse>> {
        let query = InvitationSummary::base_sql_with("where email = $1");
        tracing::debug!("query_by_email query: {}", query);

        let mut total_count: i64 = 0;
        let items: Vec<InvitationSummary> = sqlx::query(&query)
            .bind(email)
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(InvitationGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }
}

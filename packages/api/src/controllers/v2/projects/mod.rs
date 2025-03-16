use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json,
    },
};
use by_types::QueryResponse;
use deliberation_project::*;
use models::{
    deliberation_content::DeliberationContent,
    deliberations::{
        deliberation::Deliberation, deliberation_basic_info::DeliberationBasicInfo,
        deliberation_survey::DeliberationSurvey,
    },
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationProjectPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationProjectController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

// TODO: implement APIs
impl DeliberationProjectController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: DeliberationProjectQuery,
    ) -> Result<QueryResponse<DeliberationProjectSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationProjectSummary> = DeliberationProjectSummary::query_builder()
            .limit(param.size())
            .page(param.page())
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
}

impl DeliberationProjectController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id/surveys", get(Self::get_deliberation_survey))
            .route("/:id/basic-info", get(Self::get_deliberation_basic_info))
            .route("/:id/contents", get(Self::get_deliberation_contents))
            .route("/:id", get(Self::get_deliberation_project_by_id))
            .route("/", get(Self::get_deliberation_project))
            .with_state(self.clone()))
    }

    pub async fn get_deliberation_survey(
        State(ctrl): State<DeliberationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationProjectPath { id }): Path<DeliberationProjectPath>,
    ) -> Result<Json<DeliberationSurvey>> {
        tracing::debug!("get_deliberation_survey {:?}", id);

        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .query()
                .map(DeliberationSurvey::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_deliberation_contents(
        State(ctrl): State<DeliberationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationProjectPath { id }): Path<DeliberationProjectPath>,
    ) -> Result<Json<DeliberationContent>> {
        tracing::debug!("get_deliberation_contents {:?}", id);

        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .query()
                .map(DeliberationContent::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_deliberation_basic_info(
        State(ctrl): State<DeliberationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationProjectPath { id }): Path<DeliberationProjectPath>,
    ) -> Result<Json<DeliberationBasicInfo>> {
        tracing::debug!("get_deliberation_basic_info {:?}", id);

        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .query()
                .map(DeliberationBasicInfo::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_deliberation_project_by_id(
        State(ctrl): State<DeliberationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationProjectPath { id }): Path<DeliberationProjectPath>,
    ) -> Result<Json<DeliberationProject>> {
        tracing::debug!("get_deliberation_project {:?}", id);

        Ok(Json(
            DeliberationProject::query_builder()
                .id_equals(id)
                .query()
                .map(DeliberationProject::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_deliberation_project(
        State(ctrl): State<DeliberationProjectController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationProjectParam>,
    ) -> Result<Json<DeliberationProjectGetResponse>> {
        tracing::debug!("list_deliberation_project {:?}", q);

        match q {
            DeliberationProjectParam::Query(param) => Ok(Json(
                DeliberationProjectGetResponse::Query(ctrl.query(auth, param).await?),
            )),
            // DeliberationProjectParam::Read(param)
            //     if param.action == Some(DeliberationProjectReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(DeliberationProjectGetResponse::Read(res)))
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::ProjectArea;

    use crate::tests::{setup, TestContext};
    #[tokio::test]
    async fn test_get_deliberation_basic_info() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let org_id = user.orgs[0].id;

        let cli = Deliberation::get_client(&endpoint);
        let res = cli
            .create(
                org_id,
                now,
                now + 1000,
                ProjectArea::City,
                format!("title"),
                format!("test description {now}"),
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

        let deliberation = res.unwrap();
        let id = deliberation.id;

        let cli = DeliberationBasicInfo::get_client(&endpoint);
        let res = cli.read(id).await;
        assert!(res.is_ok());

        let basic_info = res.unwrap();

        assert_eq!(basic_info.id, deliberation.id);
        assert_eq!(basic_info.description, format!("test description {now}"));
    }

    #[tokio::test]
    async fn test_get_deliberation_survey() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let org_id = user.orgs[0].id;

        let cli = Deliberation::get_client(&endpoint);
        let res = cli
            .create(
                org_id,
                now,
                now + 1000,
                ProjectArea::City,
                format!("title"),
                format!("test description"),
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

        let deliberation = res.unwrap();
        let id = deliberation.id;

        let cli = DeliberationSurvey::get_client(&endpoint);
        let res = cli.read(id).await;
        assert!(res.is_ok());

        let survey = res.unwrap();

        assert_eq!(survey.id, deliberation.id);
    }

    #[tokio::test]
    async fn test_query_project() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let _now = now;
        let _org_id = user.orgs[0].id;

        let cli = DeliberationProject::get_client(&endpoint);

        let res = cli
            .query(DeliberationProjectQuery {
                size: 1,
                bookmark: None,
            })
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_get_project() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();
        let org_id = user.orgs[0].id;

        let cli = Deliberation::get_client(&endpoint);

        let res = cli
            .create(
                org_id,
                now,
                now + 1000,
                ProjectArea::City,
                format!("title"),
                format!("test description"),
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

        let deliberation = res.unwrap();

        let id = deliberation.id;

        let cli = DeliberationProject::get_client(&endpoint);
        let res = cli.get(id).await;
        assert!(res.is_ok());

        let deliberation_project = res.unwrap();

        assert_eq!(deliberation_project.id, deliberation.id);
    }
}

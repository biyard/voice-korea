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
        deliberation_draft::DeliberationDraft, deliberation_survey::DeliberationSurvey,
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
    async fn search(
        &self,
        _auth: Option<Authorization>,
        DeliberationProjectQuery { title, .. }: DeliberationProjectQuery,
    ) -> Result<QueryResponse<DeliberationProjectSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationProjectSummary> = DeliberationProjectSummary::query_builder()
            .title_contains(title.unwrap_or_default())
            .with_count()
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;
                total_count = row.get("total_count");
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn custom_query(
        &self,
        _auth: Option<Authorization>,
        param: ProjectQueryBy,
    ) -> Result<QueryResponse<DeliberationProjectSummary>> {
        let mut total_count = 0;

        let mut builder = DeliberationProjectSummary::query_builder()
            .limit(100)
            .page(1);

        if param.sorter == ProjectSorter::Newest {
            builder = builder.order_by_created_at_desc();
        } else {
            builder = builder.order_by_created_at_asc();
        }

        let items: Vec<DeliberationProjectSummary> = builder
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

    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: DeliberationProjectQuery,
    ) -> Result<QueryResponse<DeliberationProjectSummary>> {
        let mut total_count = 0;

        let builder = DeliberationProjectSummary::query_builder()
            .limit(param.size())
            .page(param.page());

        let items: Vec<DeliberationProjectSummary> = builder
            .order_by_created_at_desc()
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
            .route("/:id/draft", get(Self::get_deliberation_draft))
            .route("/:id", get(Self::get_deliberation_project_by_id))
            .route("/", get(Self::get_deliberation_project))
            .with_state(self.clone()))
    }

    pub async fn get_deliberation_draft(
        State(ctrl): State<DeliberationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationProjectPath { id }): Path<DeliberationProjectPath>,
    ) -> Result<Json<DeliberationDraft>> {
        tracing::debug!("get_deliberation_draft {:?}", id);

        Ok(Json(
            DeliberationDraft::query_builder()
                .id_equals(id)
                .query()
                .map(DeliberationDraft::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
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
            DeliberationProjectParam::Query(param) => match param.action {
                Some(DeliberationProjectQueryActionType::Search) => Ok(Json(
                    DeliberationProjectGetResponse::Query(ctrl.search(auth, param).await?),
                )),
                _ => Ok(Json(DeliberationProjectGetResponse::Query(
                    ctrl.query(auth, param).await?,
                ))),
            },
            DeliberationProjectParam::Custom(param) => Ok(Json(
                DeliberationProjectGetResponse::Query(ctrl.custom_query(auth, param).await?),
            )),
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
                "".to_string(),
                format!("title"),
                format!("test description {now}"),
                ProjectArea::City,
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
                "".to_string(),
                format!("title"),
                format!("test description"),
                ProjectArea::City,
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
                action: None,
                title: None,
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
                "".to_string(),
                format!("title"),
                format!("test description"),
                ProjectArea::City,
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

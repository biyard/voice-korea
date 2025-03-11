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
    deliberations::{deliberation::Deliberation, deliberation_basic_info::DeliberationBasicInfo},
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
            .route("/:id/basic-info", get(Self::get_deliberation_basic_info))
            .route("/:id", get(Self::get_deliberation_project_by_id))
            .route("/", get(Self::get_deliberation_project))
            .with_state(self.clone()))
    }

    pub async fn get_deliberation_basic_info(
        State(ctrl): State<DeliberationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationProjectPath { id }): Path<DeliberationProjectPath>,
    ) -> Result<Json<DeliberationBasicInfo>> {
        tracing::debug!("get_deliberation_project {:?}", id);

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
    use by_axum::axum::body::{to_bytes, Body};
    use by_axum::axum::http::{Request, Response, StatusCode};
    use models::{
        deliberations::{
            deliberation::Deliberation, deliberation_basic_info::DeliberationBasicInfo,
        },
        ProjectArea,
    };

    use crate::tests::{setup, TestContext};
    #[tokio::test]
    async fn test_get_deliberation_basic_info() {
        let TestContext {
            user,
            app,
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

        let basic_info = deliberation.clone();

        // TODO: complete this test code
        // let request = Request::builder()
        //     .method("GET")
        //     .uri(format!(
        //         "{}/v2/projects/{}/basic-info",
        //         endpoint, created_deliberation.id
        //     ))
        //     .header("Content-Type", "application/json")
        //     .body(Body::empty())
        //     .unwrap();

        // let response = app.handle(request).await.unwrap();
        // assert_eq!(response.status(), StatusCode::OK);

        // let body = to_bytes(response.into_body()).await.unwrap();
        // let basic_info: DeliberationBasicInfo = serde_json::from_slice(&body).unwrap();

        assert_eq!(basic_info.id, deliberation.id);
        assert_eq!(basic_info.description, format!("test description {now}"));
    }
}

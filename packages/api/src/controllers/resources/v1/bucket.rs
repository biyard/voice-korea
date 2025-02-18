#![allow(unused)]

use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json, Router,
    },
};

use models::{
    // ResourceDeleteRequest,
    ApiError,
    GetObjectUriRequest,
    GetObjectUriResponse,
    Resource,
    ResourceAction,
    ResourceByIdAction,
    ResourceCreateRequest,
    ResourceGetResponse,
    ResourceParam,
    ResourceReadAction,
    ResourceRepository,
    ResourceUpdateRequest,
};
use serde::{Deserialize, Serialize};

use super::ResourceControllerV1;

#[derive(Clone, Debug)]
pub struct MetadataControllerV1 {}

impl MetadataControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> models::Result<Router> {
        let ctrl = Self {};

        Ok(Router::new()
            .route("/put-uri", post(Self::get_put_object_uri))
            .with_state(ctrl))
    }

    pub async fn get_put_object_uri(
        State(ctrl): State<MetadataControllerV1>,
        Json(req): Json<GetObjectUriRequest>,
    ) -> models::Result<Json<GetObjectUriResponse>> {
        use aws_config::BehaviorVersion;
        use aws_config::{defaults, Region};
        use aws_sdk_s3::config::Credentials;
        use aws_sdk_s3::presigning::PresigningConfig;
        use uuid::Uuid;

        tracing::debug!("get_put_object_uri {:?}", req);
        let config = defaults(BehaviorVersion::latest())
            .region(Region::new(
                option_env!("AWS_REGION").unwrap_or("ap-northeast-2"),
            ))
            .credentials_provider(Credentials::new(
                env!("AWS_ACCESS_KEY_ID"),
                env!("AWS_SECRET_ACCESS_KEY"),
                None,
                None,
                "voice-korea",
            ));

        let config = config.load().await;

        let client = aws_sdk_s3::Client::new(&config);

        tracing::debug!("/aws/s3/put-uri: {:?}", req);
        const EXPIRES: u64 = 60 * 60;
        let mut presigned_uris = vec![];
        let mut uris = vec![];

        tracing::debug!("{}", env!("VOICEKOREA_BUCKET"));
        for filename in req.filenames {
            let f: Vec<&str> = filename.split(".").collect();
            let id = Uuid::new_v4();
            let key = if f.len() > 1 {
                format!("{}/{}.{}", "metadata", id, f[f.len() - 1])
            } else {
                format!("{}/{}", "metadata", id)
            };

            let presigned_request = client
                .put_object()
                .bucket(env!("VOICEKOREA_BUCKET"))
                .key(key.clone())
                .presigned(
                    PresigningConfig::expires_in(std::time::Duration::from_secs(EXPIRES)).map_err(
                        |e| {
                            tracing::error!("Failed to set expired time");
                            ApiError::SetExpiredTimeFailed
                        },
                    )?,
                )
                .await
                .map_err(|e| {
                    tracing::error!("Failed to put object");
                    ApiError::PutObjectFailed
                })?;
            presigned_uris.push(presigned_request.uri().to_string());
            uris.push(format!(
                "{}/{}",
                option_env!("VOICEKOREA_METADATA_ENDPOINT").unwrap_or_default(),
                key,
            ));
        }

        Ok(Json(GetObjectUriResponse {
            presigned_uris,
            uris,
        }))
    }
}

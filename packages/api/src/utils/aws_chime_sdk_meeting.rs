#![allow(dead_code)]
use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_chimesdkmediapipelines::{
    types::{MediaPipelineSinkType, MediaPipelineSourceType},
    Client as MediaPipelinesClient, //Error as MediaPipelinesError,
};
use aws_sdk_chimesdkmeetings::{
    //Error as MeetingsError
    types::{Attendee, Meeting},
    Client as MeetingsClient,
};
use models::{dto::MeetingInfo, error::ApiError};

#[derive(Debug)]
pub struct AttendeeInfo {
    pub attendee_id: String,
    pub external_user_id: String,
    pub join_token: String,
}

pub struct ChimeMeetingService {
    client: MeetingsClient,
    pipeline: MediaPipelinesClient,
}

impl ChimeMeetingService {
    pub async fn new() -> Self {
        let config = load_defaults(BehaviorVersion::latest()).await;
        let client = MeetingsClient::new(&config);
        let pipeline = MediaPipelinesClient::new(&config);
        Self { client, pipeline }
    }

    pub async fn get_meeting_info(&self, meeting_id: &str) -> Result<Meeting, ApiError> {
        let meeting = match self
            .client
            .get_meeting()
            .meeting_id(meeting_id)
            .send()
            .await
        {
            Ok(v) => v.meeting.unwrap(),
            Err(e) => {
                tracing::error!("get_meeting error: {:?}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        Ok(meeting)
    }

    pub async fn get_attendee_info(
        &self,
        meeting_id: &str,
        attendee_id: &str,
    ) -> Result<Attendee, ApiError> {
        let attendee = match self
            .client
            .get_attendee()
            .meeting_id(meeting_id)
            .attendee_id(attendee_id)
            .send()
            .await
        {
            Ok(v) => v.attendee.unwrap(),
            Err(e) => {
                tracing::error!("get_attendee error: {:?}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        Ok(attendee)
    }

    pub async fn create_meeting(&self, meeting_name: &str) -> Result<Meeting, ApiError> {
        let _ = meeting_name;
        let client_request_token = uuid::Uuid::new_v4().to_string();
        let conf = crate::config::get();

        let resp = match self
            .client
            .create_meeting()
            .client_request_token(client_request_token.clone())
            .external_meeting_id(client_request_token.clone())
            .media_region(conf.aws.region)
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("create_meeting error: {:?}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        let meeting = match resp.meeting {
            Some(v) => v,
            None => {
                tracing::error!("create_meeting error: no meeting");
                return Err(ApiError::AwsChimeError("no meeting".to_string()));
            }
        };

        Ok(meeting)
    }

    pub async fn create_attendee(
        &self,
        meeting: &MeetingInfo,
        external_user_id: &str,
    ) -> Result<AttendeeInfo, ApiError> {
        let resp = match self
            .client
            .create_attendee()
            .external_user_id(external_user_id)
            .meeting_id(meeting.meeting_id.clone())
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("create_attendee error: {:?}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        let attendee = match resp.attendee {
            Some(v) => v,
            None => {
                tracing::error!("create_attendee error: no attendee");
                return Err(ApiError::AwsChimeError("no attendee".to_string()));
            }
        };

        Ok(AttendeeInfo {
            attendee_id: attendee.attendee_id.unwrap_or_default(),
            external_user_id: attendee.external_user_id.unwrap_or_default(),
            join_token: attendee.join_token.unwrap_or_default(),
        })
    }

    pub async fn end_meeting(&self, meeting: &Meeting) -> Result<(), ApiError> {
        let resp = match self
            .client
            .delete_meeting()
            .meeting_id(meeting.meeting_id.clone().unwrap_or_default())
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("delete_meeting error: {:?}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        tracing::debug!("delete_meeting response: {:?}", resp);

        Ok(())
    }

    pub async fn make_pipeline(
        &self,
        meeting: Meeting,
        _meeting_name: String,
    ) -> Result<String, ApiError> {
        // FIXME: Use env var
        // let bucket_name = std::env::var("AWS_MEDIA_PIPELINE_BUCKET_NAME").unwrap();
        let bucket_name = crate::config::get().chime_bucket_name.to_string();

        let client_request_token = uuid::Uuid::new_v4().to_string();

        let resp = match self
            .pipeline
            .create_media_capture_pipeline()
            .client_request_token(client_request_token)
            .source_type(MediaPipelineSourceType::ChimeSdkMeeting)
            .source_arn(meeting.meeting_arn.unwrap_or_default())
            .sink_type(MediaPipelineSinkType::S3Bucket)
            .sink_arn(format!("arn:aws:s3:::{}", bucket_name))
            // .chime_sdk_meeting_configuration(sink_configuration)
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("create_media_capture_pipeline error: {:?}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        tracing::debug!("create_media_capture_pipeline response: {:?}", resp);

        let pipeline_id = resp
            .media_capture_pipeline
            .as_ref()
            .and_then(|p| p.media_pipeline_id.clone())
            .unwrap_or_default();

        Ok(pipeline_id)
        // Ok(pipeline_id)
    }

    pub async fn end_pipeline(&self, pipeline_id: &str) -> Result<(), ApiError> {
        let resp = match self
            .pipeline
            .delete_media_capture_pipeline()
            .media_pipeline_id(pipeline_id)
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("delete_media_capture_pipeline error: {:?}", e);
                return Err(ApiError::AwsChimeError(e.to_string()));
            }
        };

        tracing::debug!("delete_media_capture_pipeline response: {:?}", resp);

        Ok(())
    }
}

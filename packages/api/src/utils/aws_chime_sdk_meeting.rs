#![allow(dead_code)]
use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_chimesdkmediapipelines::{
    types::{MediaPipelineSinkType, MediaPipelineSourceType},
    Client as MediaPipelinesClient, //Error as MediaPipelinesError,
};
use aws_sdk_chimesdkmeetings::{
    types::MediaPlacement,
    //Error as MeetingsError
    Client as MeetingsClient,
};
use models::error::ApiError;

#[derive(Debug)]
pub struct MeetingInfo {
    pub id: String,
    pub name: String,
    pub media_region: String,
    pub arn: String,
    pub client_request_token: String,
    pub media_placement: Option<MediaPlacement>,
    pub attendees: Vec<AttendeeInfo>,
}

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

    pub async fn create_meeting(&self, meeting_name: &str) -> Result<MeetingInfo, ApiError> {
        let client_request_token = uuid::Uuid::new_v4().to_string();

        let resp = match self
            .client
            .create_meeting()
            .client_request_token(client_request_token.clone())
            .external_meeting_id(meeting_name)
            .media_region("ap-northeast-2") // FIXME: Use env var
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

        Ok(MeetingInfo {
            id: meeting.meeting_id.unwrap_or_default(),
            name: meeting
                .external_meeting_id
                .unwrap_or(meeting_name.to_string()),
            media_region: meeting.media_region.unwrap_or("ap-northeast-2".to_string()),
            media_placement: meeting.media_placement,
            client_request_token,
            arn: meeting.meeting_arn.unwrap_or_default(),
            attendees: vec![],
        })
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
            .meeting_id(meeting.id.clone())
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

    pub async fn end_meeting(&self, meeting: &MeetingInfo) -> Result<(), ApiError> {
        let resp = match self
            .client
            .delete_meeting()
            .meeting_id(meeting.id.clone())
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

    pub async fn make_pipeline(&self, meeting: MeetingInfo) -> Result<String, ApiError> {
        // FIXME: Use env var
        // let bucket_name = std::env::var("AWS_MEDIA_PIPELINE_BUCKET_NAME").unwrap();
        let bucket_name = "voicekorea-chime".to_string(); // for testing
        let object_key = format!("recordings/{}#{}", meeting.name, meeting.id);
        let client_request_token = uuid::Uuid::new_v4().to_string();

        // let sink_configuration = ChimeSdkMeetingConfiguration::builder()
        //     .source_configuration()
        //     .build()?
        //     .artifacts_configuration(
        //         aws_sdk_chimesdkmediapipelines::types::ArtifactsConfiguration::builder()
        //             .audio(
        //                 aws_sdk_chimesdkmediapipelines::types::AudioArtifactsConfiguration::builder()
        //                     .mux_type("AudioWithCompositeMux")
        //                     .build()?
        //             )
        //             .video(
        //                 aws_sdk_chimesdkmediapipelines::types::VideoArtifactsConfiguration::builder()
        //                     .mux_type("VideoWithCompositeMux")
        //                     .build()?
        //             )
        //             .content(
        //                 aws_sdk_chimesdkmediapipelines::types::ContentArtifactsConfiguration::builder()
        //                     .mux_type("ContentWithCompositeMux")
        //                     .state("Enabled")
        //                     .build()?
        //             )
        //             .composite_video(
        //                 aws_sdk_chimesdkmediapipelines::types::CompositeArtifactsConfiguration::builder()
        //                     .layout("GridView")
        //                     .resolution("HD")
        //                     .build()?
        //             )
        //             .build()?
        //     )
        //     .build()?;

        let resp = match self
            .pipeline
            .create_media_capture_pipeline()
            .client_request_token(client_request_token)
            .source_type(MediaPipelineSourceType::ChimeSdkMeeting)
            .source_arn(meeting.arn)
            .sink_type(MediaPipelineSinkType::S3Bucket)
            .sink_arn(format!("arn:aws:s3:::{}/{}", bucket_name, object_key))
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

        Ok("pipeline_id".to_string())
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

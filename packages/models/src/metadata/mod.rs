use crate::field::Field;
#[cfg(feature = "server")]
use by_axum::aide;
// use by_macros::api_model;
use dioxus_translate::Language;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MetadataRequest {
    pub file_name: String,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GetPutObjectUriRequest {
    pub file_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GetPutObjectUriResponse {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreateMetadataRequest {
    pub name: String,
    pub urls: Vec<String>,
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<Field>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,

    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UpdateMetadataRequest {
    pub name: String,
    pub urls: Vec<String>,
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<Field>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,
    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
}

impl From<ResourceMetadata> for UpdateMetadataRequest {
    fn from(resource: ResourceMetadata) -> Self {
        Self {
            name: resource.name.clone(),
            urls: resource.urls.clone(),
            metadata_type: resource.metadata_type.clone(),
            metadata_field: resource.metadata_field.clone(),
            metadata_purpose: resource.metadata_purpose.clone(),
            metadata_source: resource.metadata_source.clone(),
            metadata_authority: resource.metadata_authority.clone(),
            public_opinion_projects: resource.public_opinion_projects.clone(),
            public_survey_projects: resource.public_survey_projects.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// #[api_model(base = "/metadatas/v2", read_action = [get_resource] , table = resource, iter_type=QueryResponse)]
pub struct ResourceMetadata {
    // #[api_model(summary, primary_key)]
    pub id: String,
    pub name: String,
    pub urls: Vec<String>, // need to be divide table when postgre db implemented (resource url couldn't be name)
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<Field>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,

    // #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    // #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
}

impl ResourceMetadata {
    pub fn new(
        name: String,
        urls: Vec<String>,
        data_type: Option<MetadataType>,
        field: Option<Field>,
        purpose: Option<MetadataPurpose>,
        source: Option<MetadataSource>,
        authority: Option<MetadataAuthority>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            urls,
            metadata_type: data_type,
            metadata_field: field,
            metadata_purpose: purpose,
            metadata_source: source,
            metadata_authority: authority,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            deleted_at: None,
            public_opinion_projects: None,
            public_survey_projects: None,
        }
    }
}

// TODO: implement metadata - projects linking table

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataActionRequest {
    Create(CreateMetadataRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataByIdActionRequest {
    Delete,
    Update(UpdateMetadataRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicOpinion {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurvey {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataAuthority {
    #[default]
    Public,
    Private,
    Restricted,
}

impl MetadataAuthority {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                MetadataAuthority::Public => "Public Material",
                MetadataAuthority::Private => "Private Material",
                MetadataAuthority::Restricted => "Restricted Material",
            },
            Language::Ko => match self {
                MetadataAuthority::Public => "공개 자료",
                MetadataAuthority::Private => "기밀 자료",
                MetadataAuthority::Restricted => "제한 자료",
            },
        }
    }
}

impl FromStr for MetadataAuthority {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "공개 자료" | "Public Material" => Ok(MetadataAuthority::Public),
            "기밀 자료" | "Private Material" => Ok(MetadataAuthority::Private),
            "제한 자료" | "Restricted Material" => Ok(MetadataAuthority::Restricted),
            _ => Err(format!("invalid field")),
        }
    }

    type Err = String;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataSource {
    #[default]
    Internal,
    External,
    Government,
    Company,
}

impl MetadataSource {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                MetadataSource::Internal => "Internal Material",
                MetadataSource::External => "External Material",
                MetadataSource::Government => "Agency",
                MetadataSource::Company => "Privacy Enterprise",
            },
            Language::Ko => match self {
                MetadataSource::Internal => "내부 자료",
                MetadataSource::External => "외부 자료",
                MetadataSource::Government => "정부 기관",
                MetadataSource::Company => "민간 기업",
            },
        }
    }
}

impl FromStr for MetadataSource {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "내부 자료" | "Internal Material" => Ok(MetadataSource::Internal),
            "외부 자료" | "External Material" => Ok(MetadataSource::External),
            "정부 기관" | "Agency" => Ok(MetadataSource::Government),
            "민간 기업" | "Privacy Enterprise" => Ok(MetadataSource::Company),
            _ => Err(format!("invalid field")),
        }
    }

    type Err = String;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataPurpose {
    #[default]
    DevelopmentPolicy,
    AcademicResearch,
    PublicDiscussion,
    Education,
}

impl MetadataPurpose {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                MetadataPurpose::DevelopmentPolicy => "Policy Development",
                MetadataPurpose::AcademicResearch => "Academic Research",
                MetadataPurpose::PublicDiscussion => "Public Discussion Document",
                MetadataPurpose::Education => "Education Document",
            },
            Language::Ko => match self {
                MetadataPurpose::DevelopmentPolicy => "정책 개발",
                MetadataPurpose::AcademicResearch => "학술 연구",
                MetadataPurpose::PublicDiscussion => "공론화 자료",
                MetadataPurpose::Education => "교육 자료",
            },
        }
    }
}

impl FromStr for MetadataPurpose {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "정책 개발" | "Policy Development" => Ok(MetadataPurpose::DevelopmentPolicy),
            "학술 연구" | "Academic Research" => Ok(MetadataPurpose::AcademicResearch),
            "공론화 자료" | "Public Discussion Document" => {
                Ok(MetadataPurpose::PublicDiscussion)
            }
            "교육 자료" | "Education Document" => Ok(MetadataPurpose::Education),
            _ => Err(format!("invalid field")),
        }
    }

    type Err = String;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum MetadataType {
    #[default]
    Report,
    Statistics,
    Survey,
    Thesis,
    Presentation,
    Media,
}

impl MetadataType {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                MetadataType::Report => "Report",
                MetadataType::Statistics => "Statistics",
                MetadataType::Survey => "Survey",
                MetadataType::Thesis => "Thesis",
                MetadataType::Presentation => "Presentations",
                MetadataType::Media => "Media",
            },
            Language::Ko => match self {
                MetadataType::Report => "보고서",
                MetadataType::Statistics => "통계 자료",
                MetadataType::Survey => "설문 데이터",
                MetadataType::Thesis => "연구 논문",
                MetadataType::Presentation => "발표 자료",
                MetadataType::Media => "미디어",
            },
        }
    }
}

impl FromStr for MetadataType {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "보고서" | "Report" => Ok(MetadataType::Report),
            "통계 자료" | "Statistics" => Ok(MetadataType::Statistics),
            "설문 데이터" | "Survey" => Ok(MetadataType::Survey),
            "연구 논문" | "Thesis" => Ok(MetadataType::Thesis),
            "발표 자료" | "Presentations" => Ok(MetadataType::Presentation),
            "미디어" | "Media" => Ok(MetadataType::Media),
            _ => Err(format!("invalid field")),
        }
    }

    type Err = String;
}

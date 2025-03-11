#![allow(unused)]

use crate::ProjectArea;
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;
use dioxus_translate::Translate;
use serde::{Deserialize, Serialize};

use by_types::ApiError;

#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct GetObjectUriResponse {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct GetObjectUriRequest {
    pub filenames: Vec<String>,
}

#[derive(validator::Validate)]
///FIXME: fix to filenames to type vector
#[api_model(base = "/v2/organizations/:org_id/resources", table = resources, iter_type=QueryResponse)]
pub struct ResourceFile {
    #[api_model(summary, primary_key, action = delete, read_action = find_by_id )]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update, query_action = search_by)]
    pub title: String,

    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub resource_type: Option<ResourceType>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub project_area: Option<ProjectArea>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub usage_purpose: Option<UsagePurpose>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub source: Option<Source>,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub access_level: Option<AccessLevel>,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    pub files: Vec<File>,
    // TODO: After Implement Deliberation Table
    // #[api_model(many_to_many = resource_delierations, foreign_table_name = delierations, foreign_primary_key = delieration_id, foreign_reference_key = resource_id)]
    // pub deliberations: Option<Vec<Deliberation>>,

    // TODO: After Implement Survey Table
    // #[api_model(many_to_many = resource_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = resource_id)]
    // pub surveys: Option<Vec<Survey>>,

    // FIXME: "one_to_many" is not supported yet
    // #[api_model(one_to_many = metadatas, foreign_key = resource_id)]
    // #[serde(default)]
    // pub files: Vec<Metadata>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct File {
    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum FileExtension {
    JPG = 1,
    PNG = 2,
    PDF = 3,
    ZIP = 4,
    WORD = 5,
    PPTX = 6,
    EXCEL = 7,
}

impl FileExtension {
    pub fn from_str(s: &str) -> Result<FileExtension> {
        match s {
            "jpg" | "jpeg" => Ok(FileExtension::JPG),
            "png" => Ok(FileExtension::PNG),
            "pdf" => Ok(FileExtension::PDF),
            "zip" => Ok(FileExtension::ZIP),
            "doc" | "docx" => Ok(FileExtension::WORD),
            "ppt" | "pptx" => Ok(FileExtension::PPTX),
            "xls" | "xlsx" => Ok(FileExtension::EXCEL),
            _ => Err(crate::ApiError::InvalidType),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ResourceType {
    #[default]
    #[translate(ko = "보고서")]
    Report = 1,
    #[translate(ko = "통계 자료")]
    Statistics = 2,
    #[translate(ko = "설문 데이터")]
    Survey = 3,
    #[translate(ko = "연구 논문")]
    Thesis = 4,
    #[translate(ko = "발표 자료")]
    Presentation = 5,
    #[translate(ko = "미디어")]
    Media = 6,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum UsagePurpose {
    #[default]
    #[translate(ko = "정책 개발")]
    PolicyDevelopment = 1,
    #[translate(ko = "학술 연구")]
    AcademicResearch = 2,
    #[translate(ko = "공론화 자료")]
    PublicDebate = 3,
    #[translate(ko = "교육 자료")]
    EducationalMaterial = 4,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Source {
    #[default]
    #[translate(ko = "내부 자료")]
    Internal = 1,
    #[translate(ko = "외부 자료")]
    External = 2,
    #[translate(ko = "정부 기관")]
    Government = 3,
    #[translate(ko = "민간 기업")]
    Company = 4,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AccessLevel {
    #[default]
    #[translate(ko = "공개 자료")]
    Public = 1,
    #[translate(ko = "제한 자료")]
    Restricted = 2,
    #[translate(ko = "기밀 자료")]
    Confidential = 3,
}

// TODO: After Implement "One-to-Many" Relationship

// #[api_model(base = "/resource/v1/metadata", table = metadatas, iter_type=QueryResponse)]
// pub struct Metadata {
//     #[api_model(primary_key, read_action = find_by_id)]
//     pub id: String,
//     #[api_model(auto = insert)]
//     pub created_at: i64,
//     #[api_model(auto = [insert, update])]
//     pub updated_at: i64,
//     #[api_model(action = create)]
//     pub url: String,
//     #[api_model(action = create)]
//     pub format: Format,
//     // FIXME: "one_to_many" is not supported yet
//     // #[api_model(many_to_one = resources)]
//     // pub resource_id: String,
// }

// #[derive(Debug, Default, Clone, Eq, PartialEq, ApiModel)]
// #[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
// pub enum Format {
//     #[default]
//     PDF = 1,
//     Excel = 2,
//     Word = 3,
//     Media = 4,
// }

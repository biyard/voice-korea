use models::{ProjectArea, ResourceFile, SurveyV2Summary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliberationInformation {
    pub deliberation_type: Option<ProjectArea>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub documents: Vec<ResourceFile>,
    pub projects: Vec<SurveyV2Summary>,
}

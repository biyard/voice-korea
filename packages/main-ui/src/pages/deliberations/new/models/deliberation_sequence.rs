use models::step_type::StepType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct DeliberationSequence {
    pub name: String,
    pub start_date: Option<u64>,
    pub end_date: Option<u64>,
    pub step_type: Option<StepType>,
}

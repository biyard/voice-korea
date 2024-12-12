use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PiChart {
    pub label: String,
    pub percentage: f64,
    pub color: &'static str,
}

/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeOffResponse {
    #[serde(rename = "model")]
    pub model: Box<crate::models::TimeOff>,
    #[serde(rename = "warnings")]
    pub warnings: Vec<crate::models::WarningValidationProblem>,
    #[serde(rename = "errors")]
    pub errors: Vec<crate::models::ErrorValidationProblem>,
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<crate::models::DebugModeLog>>,
}

impl TimeOffResponse {
    pub fn new(model: crate::models::TimeOff, warnings: Vec<crate::models::WarningValidationProblem>, errors: Vec<crate::models::ErrorValidationProblem>) -> TimeOffResponse {
        TimeOffResponse {
            model: Box::new(model),
            warnings,
            errors,
            logs: None,
        }
    }
}



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
pub struct WarningValidationProblem {
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<crate::models::ValidationProblemSource>>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "problem_type")]
    pub problem_type: String,
}

impl WarningValidationProblem {
    pub fn new(title: String, detail: String, problem_type: String) -> WarningValidationProblem {
        WarningValidationProblem {
            source: None,
            title,
            detail,
            problem_type,
        }
    }
}


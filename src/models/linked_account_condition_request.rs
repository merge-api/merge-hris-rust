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
pub struct LinkedAccountConditionRequest {
    /// The ID indicating which condition schema to use for a specific condition.
    #[serde(rename = "condition_schema_id")]
    pub condition_schema_id: String,
    /// The operator for a specific condition.
    #[serde(rename = "operator")]
    pub operator: String,
    /// The value for a specific condition.
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
}

impl LinkedAccountConditionRequest {
    pub fn new(condition_schema_id: String, operator: String, value: Option<serde_json::Value>) -> LinkedAccountConditionRequest {
        LinkedAccountConditionRequest {
            condition_schema_id,
            operator,
            value,
        }
    }
}



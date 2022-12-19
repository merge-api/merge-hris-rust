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
pub struct OperatorSchema {
    /// The operator for which an operator schema is defined.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// Whether the operator can be repeated multiple times.
    #[serde(rename = "is_unique", skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
}

impl OperatorSchema {
    pub fn new() -> OperatorSchema {
        OperatorSchema {
            operator: None,
            is_unique: None,
        }
    }
}



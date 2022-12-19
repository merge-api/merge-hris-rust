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
pub struct MetaResponse {
    #[serde(rename = "request_schema")]
    pub request_schema: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::LinkedAccountStatus>>,
    #[serde(rename = "has_conditional_params")]
    pub has_conditional_params: bool,
    #[serde(rename = "has_required_linked_account_params")]
    pub has_required_linked_account_params: bool,
}

impl MetaResponse {
    pub fn new(request_schema: ::std::collections::HashMap<String, serde_json::Value>, has_conditional_params: bool, has_required_linked_account_params: bool) -> MetaResponse {
        MetaResponse {
            request_schema,
            status: None,
            has_conditional_params,
            has_required_linked_account_params,
        }
    }
}



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
pub struct TimeOffEndpointRequest {
    #[serde(rename = "model")]
    pub model: Box<crate::models::TimeOffRequest>,
}

impl TimeOffEndpointRequest {
    pub fn new(model: crate::models::TimeOffRequest) -> TimeOffEndpointRequest {
        TimeOffEndpointRequest {
            model: Box::new(model),
        }
    }
}



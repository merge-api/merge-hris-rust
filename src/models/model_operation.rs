/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// ModelOperation : # The ModelOperation Object ### Description The `ModelOperation` object is used to represent the operations that are currently supported for a given model.  ### Usage Example View what operations are supported for the `Candidate` endpoint.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ModelOperation {
    #[serde(rename = "model_name")]
    pub model_name: String,
    #[serde(rename = "available_operations")]
    pub available_operations: Vec<String>,
    #[serde(rename = "required_post_parameters")]
    pub required_post_parameters: Vec<String>,
    #[serde(rename = "supported_fields")]
    pub supported_fields: Vec<String>,
}

impl ModelOperation {
    /// # The ModelOperation Object ### Description The `ModelOperation` object is used to represent the operations that are currently supported for a given model.  ### Usage Example View what operations are supported for the `Candidate` endpoint.
    pub fn new(model_name: String, available_operations: Vec<String>, required_post_parameters: Vec<String>, supported_fields: Vec<String>) -> ModelOperation {
        ModelOperation {
            model_name,
            available_operations,
            required_post_parameters,
            supported_fields,
        }
    }
}



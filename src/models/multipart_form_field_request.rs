/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// MultipartFormFieldRequest : # The MultipartFormField Object ### Description The `MultipartFormField` object is used to represent fields in an HTTP request using `multipart/form-data`.  ### Usage Example Create a `MultipartFormField` to define a multipart form entry.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MultipartFormFieldRequest {
    /// The name of the form field
    #[serde(rename = "name")]
    pub name: String,
    /// The data for the form field.
    #[serde(rename = "data")]
    pub data: String,
    /// The encoding of the value of `data`. Defaults to `RAW` if not defined.
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Box<crate::models::EncodingEnum>>,
    /// The file name of the form field, if the field is for a file.
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The MIME type of the file, if the field is for a file.
    #[serde(rename = "content_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl MultipartFormFieldRequest {
    /// # The MultipartFormField Object ### Description The `MultipartFormField` object is used to represent fields in an HTTP request using `multipart/form-data`.  ### Usage Example Create a `MultipartFormField` to define a multipart form entry.
    pub fn new(name: String, data: String) -> MultipartFormFieldRequest {
        MultipartFormFieldRequest {
            name,
            data,
            encoding: None,
            file_name: None,
            content_type: None,
        }
    }
}



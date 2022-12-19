/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// GenerateRemoteKeyRequest : # The GenerateRemoteKey Object ### Description The `GenerateRemoteKey` object is used to represent a request for a new remote key.  ### Usage Example Post a `GenerateRemoteKey` to create a new remote key.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateRemoteKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
}

impl GenerateRemoteKeyRequest {
    /// # The GenerateRemoteKey Object ### Description The `GenerateRemoteKey` object is used to represent a request for a new remote key.  ### Usage Example Post a `GenerateRemoteKey` to create a new remote key.
    pub fn new(name: String) -> GenerateRemoteKeyRequest {
        GenerateRemoteKeyRequest {
            name,
        }
    }
}


/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// SyncStatus : # The SyncStatus Object ### Description The `SyncStatus` object is used to represent the syncing state of an account  ### Usage Example View the `SyncStatus` for an account to see how recently its models were synced.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SyncStatus {
    #[serde(rename = "model_name")]
    pub model_name: String,
    #[serde(rename = "model_id")]
    pub model_id: String,
    #[serde(rename = "last_sync_start", skip_serializing_if = "Option::is_none")]
    pub last_sync_start: Option<String>,
    #[serde(rename = "next_sync_start", skip_serializing_if = "Option::is_none")]
    pub next_sync_start: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<Box<crate::models::SyncStatusStatusEnum>>,
    #[serde(rename = "is_initial_sync")]
    pub is_initial_sync: bool,
    #[serde(rename = "selective_sync_configurations_usage", skip_serializing_if = "Option::is_none")]
    pub selective_sync_configurations_usage: Option<Box<crate::models::SelectiveSyncConfigurationsUsageEnum>>,
}

impl SyncStatus {
    /// # The SyncStatus Object ### Description The `SyncStatus` object is used to represent the syncing state of an account  ### Usage Example View the `SyncStatus` for an account to see how recently its models were synced.
    pub fn new(model_name: String, model_id: String, status: Option<crate::models::SyncStatusStatusEnum>, is_initial_sync: bool) -> SyncStatus {
        SyncStatus {
            model_name,
            model_id,
            last_sync_start: None,
            next_sync_start: None,
            status: Box::new(status),
            is_initial_sync,
            selective_sync_configurations_usage: None,
        }
    }
}



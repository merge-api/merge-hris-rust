/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// TimeOff : # The TimeOff Object ### Description The `TimeOff` object is used to represent a Time Off Request filed by an employee.  ### Usage Example Fetch from the `LIST TimeOffs` endpoint and filter by `ID` to show all time off requests.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeOff {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The third-party API ID of the matching object.
    #[serde(rename = "remote_id", skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// The employee requesting time off.
    #[serde(rename = "employee", skip_serializing_if = "Option::is_none")]
    pub employee: Option<String>,
    /// The employee approving the time off request.
    #[serde(rename = "approver", skip_serializing_if = "Option::is_none")]
    pub approver: Option<String>,
    /// The status of this time off request.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::TimeOffStatusEnum>>,
    /// The employee note for this time off request.
    #[serde(rename = "employee_note", skip_serializing_if = "Option::is_none")]
    pub employee_note: Option<String>,
    /// The unit of time requested.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<Box<crate::models::UnitsEnum>>,
    /// The number of time off units requested.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// The type of time off request.
    #[serde(rename = "request_type", skip_serializing_if = "Option::is_none")]
    pub request_type: Option<Box<crate::models::RequestTypeEnum>>,
    /// The day and time of the start of the time requested off.
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The day and time of the end of the time requested off.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "remote_data", skip_serializing_if = "Option::is_none")]
    pub remote_data: Option<Vec<crate::models::RemoteData>>,
    #[serde(rename = "remote_was_deleted", skip_serializing_if = "Option::is_none")]
    pub remote_was_deleted: Option<bool>,
    #[serde(rename = "field_mappings", skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl TimeOff {
    /// # The TimeOff Object ### Description The `TimeOff` object is used to represent a Time Off Request filed by an employee.  ### Usage Example Fetch from the `LIST TimeOffs` endpoint and filter by `ID` to show all time off requests.
    pub fn new() -> TimeOff {
        TimeOff {
            id: None,
            remote_id: None,
            employee: None,
            approver: None,
            status: None,
            employee_note: None,
            units: None,
            amount: None,
            request_type: None,
            start_time: None,
            end_time: None,
            remote_data: None,
            remote_was_deleted: None,
            field_mappings: None,
        }
    }
}


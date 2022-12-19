/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// Benefit : # The Benefit Object ### Description The `Benefit` object is used to represent a Benefit for an employee.  ### Usage Example Fetch from the `LIST Benefits` endpoint and filter by `ID` to show all benefits.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Benefit {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The third-party API ID of the matching object.
    #[serde(rename = "remote_id", skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// The employee on the plan.
    #[serde(rename = "employee", skip_serializing_if = "Option::is_none")]
    pub employee: Option<String>,
    /// The name of the benefit provider.
    #[serde(rename = "provider_name", skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// The type of benefit plan
    #[serde(rename = "benefit_plan_type", skip_serializing_if = "Option::is_none")]
    pub benefit_plan_type: Option<String>,
    /// The employee's contribution.
    #[serde(rename = "employee_contribution", skip_serializing_if = "Option::is_none")]
    pub employee_contribution: Option<f32>,
    /// The company's contribution.
    #[serde(rename = "company_contribution", skip_serializing_if = "Option::is_none")]
    pub company_contribution: Option<f32>,
    #[serde(rename = "remote_data", skip_serializing_if = "Option::is_none")]
    pub remote_data: Option<Vec<crate::models::RemoteData>>,
    /// Indicates whether or not this object has been deleted by third party webhooks.
    #[serde(rename = "remote_was_deleted", skip_serializing_if = "Option::is_none")]
    pub remote_was_deleted: Option<bool>,
    #[serde(rename = "field_mappings", skip_serializing_if = "Option::is_none")]
    pub field_mappings: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl Benefit {
    /// # The Benefit Object ### Description The `Benefit` object is used to represent a Benefit for an employee.  ### Usage Example Fetch from the `LIST Benefits` endpoint and filter by `ID` to show all benefits.
    pub fn new() -> Benefit {
        Benefit {
            id: None,
            remote_id: None,
            employee: None,
            provider_name: None,
            benefit_plan_type: None,
            employee_contribution: None,
            company_contribution: None,
            remote_data: None,
            remote_was_deleted: None,
            field_mappings: None,
        }
    }
}



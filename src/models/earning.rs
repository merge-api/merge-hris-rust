/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// Earning : # The Earning Object ### Description The `Earning` object is used to represent an earning for a given employee's payroll run. One run could include several earnings.  ### Usage Example Fetch from the `LIST Earnings` endpoint and filter by `ID` to show all earnings.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Earning {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "employee_payroll_run", skip_serializing_if = "Option::is_none")]
    pub employee_payroll_run: Option<String>,
    /// The amount earned.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// The type of earning.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::models::EarningTypeEnum>>,
    /// Indicates whether or not this object has been deleted by third party webhooks.
    #[serde(rename = "remote_was_deleted", skip_serializing_if = "Option::is_none")]
    pub remote_was_deleted: Option<bool>,
}

impl Earning {
    /// # The Earning Object ### Description The `Earning` object is used to represent an earning for a given employee's payroll run. One run could include several earnings.  ### Usage Example Fetch from the `LIST Earnings` endpoint and filter by `ID` to show all earnings.
    pub fn new() -> Earning {
        Earning {
            id: None,
            employee_payroll_run: None,
            amount: None,
            _type: None,
            remote_was_deleted: None,
        }
    }
}



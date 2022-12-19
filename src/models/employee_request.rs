/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */

/// EmployeeRequest : # The Employee Object ### Description The `Employee` object is used to represent an Employee for a company.  ### Usage Example Fetch from the `LIST Employee` endpoint and filter by `ID` to show all employees.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmployeeRequest {
    /// The third-party API ID of the matching object.
    #[serde(rename = "remote_id", skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    /// The employee's number that appears in the remote UI. Note: This is distinct from the remote_id field, which is a unique identifier for the employee set by the remote API, and is not exposed to the user. This value can also change in many API providers.
    #[serde(rename = "employee_number", skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    /// The ID of the employee's company.
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The employee's first name.
    #[serde(rename = "first_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The employee's last name.
    #[serde(rename = "last_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The employee's full name, to use for display purposes. If a preferred first name is available, the full name will include the preferred first name.
    #[serde(rename = "display_full_name", skip_serializing_if = "Option::is_none")]
    pub display_full_name: Option<String>,
    /// The employee's username that appears in the remote UI.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// The employee's work email.
    #[serde(rename = "work_email", skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
    /// The employee's personal email.
    #[serde(rename = "personal_email", skip_serializing_if = "Option::is_none")]
    pub personal_email: Option<String>,
    /// The employee's mobile phone number.
    #[serde(rename = "mobile_phone_number", skip_serializing_if = "Option::is_none")]
    pub mobile_phone_number: Option<String>,
    /// Array of `Employment` IDs for this Employee.
    #[serde(rename = "employments", skip_serializing_if = "Option::is_none")]
    pub employments: Option<Vec<String>>,
    /// The employee's home address.
    #[serde(rename = "home_location", skip_serializing_if = "Option::is_none")]
    pub home_location: Option<String>,
    /// The employee's work address.
    #[serde(rename = "work_location", skip_serializing_if = "Option::is_none")]
    pub work_location: Option<String>,
    /// The employee ID of the employee's manager.
    #[serde(rename = "manager", skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    /// The employee's team.
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    /// The employee's pay group
    #[serde(rename = "pay_group", skip_serializing_if = "Option::is_none")]
    pub pay_group: Option<String>,
    /// The employee's social security number.
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The employee's gender.
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Box<crate::models::GenderEnum>>,
    /// The employee's ethnicity.
    #[serde(rename = "ethnicity", skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<Box<crate::models::EthnicityEnum>>,
    /// The employee's marital status.
    #[serde(rename = "marital_status", skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<Box<crate::models::MaritalStatusEnum>>,
    /// The employee's date of birth.
    #[serde(rename = "date_of_birth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The date that the employee was hired, usually the day that an offer letter is signed. If an employee has multiple hire dates from previous employments, this represents the most recent hire date. Note: If you're looking for the employee's start date, refer to the start_date field.
    #[serde(rename = "hire_date", skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
    /// The date that the employee started working. If an employee has multiple start dates from previous employments, this represents the most recent start date.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// When the third party's employee was created.
    #[serde(rename = "remote_created_at", skip_serializing_if = "Option::is_none")]
    pub remote_created_at: Option<String>,
    /// The employment status of the employee.
    #[serde(rename = "employment_status", skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<Box<crate::models::EmploymentStatusEnum>>,
    /// The employee's termination date.
    #[serde(rename = "termination_date", skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    /// The URL of the employee's avatar image.
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Custom fields configured for a given model.
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "integration_params", skip_serializing_if = "Option::is_none")]
    pub integration_params: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "linked_account_params", skip_serializing_if = "Option::is_none")]
    pub linked_account_params: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl EmployeeRequest {
    /// # The Employee Object ### Description The `Employee` object is used to represent an Employee for a company.  ### Usage Example Fetch from the `LIST Employee` endpoint and filter by `ID` to show all employees.
    pub fn new() -> EmployeeRequest {
        EmployeeRequest {
            remote_id: None,
            employee_number: None,
            company: None,
            first_name: None,
            last_name: None,
            display_full_name: None,
            username: None,
            groups: None,
            work_email: None,
            personal_email: None,
            mobile_phone_number: None,
            employments: None,
            home_location: None,
            work_location: None,
            manager: None,
            team: None,
            pay_group: None,
            ssn: None,
            gender: None,
            ethnicity: None,
            marital_status: None,
            date_of_birth: None,
            hire_date: None,
            start_date: None,
            remote_created_at: None,
            employment_status: None,
            termination_date: None,
            avatar: None,
            custom_fields: None,
            integration_params: None,
            linked_account_params: None,
        }
    }
}



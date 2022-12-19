# EmployeeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**employee_number** | Option<**String**> | The employee's number that appears in the remote UI. Note: This is distinct from the remote_id field, which is a unique identifier for the employee set by the remote API, and is not exposed to the user. This value can also change in many API providers. | [optional]
**company** | Option<**String**> | The ID of the employee's company. | [optional]
**first_name** | Option<**String**> | The employee's first name. | [optional]
**last_name** | Option<**String**> | The employee's last name. | [optional]
**display_full_name** | Option<**String**> | The employee's full name, to use for display purposes. If a preferred first name is available, the full name will include the preferred first name. | [optional]
**username** | Option<**String**> | The employee's username that appears in the remote UI. | [optional]
**groups** | Option<**Vec<String>**> |  | [optional]
**work_email** | Option<**String**> | The employee's work email. | [optional]
**personal_email** | Option<**String**> | The employee's personal email. | [optional]
**mobile_phone_number** | Option<**String**> | The employee's mobile phone number. | [optional]
**employments** | Option<**Vec<String>**> | Array of `Employment` IDs for this Employee. | [optional]
**home_location** | Option<**String**> | The employee's home address. | [optional]
**work_location** | Option<**String**> | The employee's work address. | [optional]
**manager** | Option<**String**> | The employee ID of the employee's manager. | [optional]
**team** | Option<**String**> | The employee's team. | [optional]
**pay_group** | Option<**String**> | The employee's pay group | [optional]
**ssn** | Option<**String**> | The employee's social security number. | [optional]
**gender** | Option<[**crate::models::GenderEnum**](GenderEnum.md)> | The employee's gender. | [optional]
**ethnicity** | Option<[**crate::models::EthnicityEnum**](EthnicityEnum.md)> | The employee's ethnicity. | [optional]
**marital_status** | Option<[**crate::models::MaritalStatusEnum**](MaritalStatusEnum.md)> | The employee's marital status. | [optional]
**date_of_birth** | Option<**String**> | The employee's date of birth. | [optional]
**hire_date** | Option<**String**> | The date that the employee was hired, usually the day that an offer letter is signed. If an employee has multiple hire dates from previous employments, this represents the most recent hire date. Note: If you're looking for the employee's start date, refer to the start_date field. | [optional]
**start_date** | Option<**String**> | The date that the employee started working. If an employee has multiple start dates from previous employments, this represents the most recent start date. | [optional]
**remote_created_at** | Option<**String**> | When the third party's employee was created. | [optional]
**employment_status** | Option<[**crate::models::EmploymentStatusEnum**](EmploymentStatusEnum.md)> | The employment status of the employee. | [optional]
**termination_date** | Option<**String**> | The employee's termination date. | [optional]
**avatar** | Option<**String**> | The URL of the employee's avatar image. | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Custom fields configured for a given model. | [optional]
**integration_params** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**linked_account_params** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



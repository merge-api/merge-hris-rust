# EmployeePayrollRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**employee** | Option<**String**> | The employee whose payroll is being run. | [optional]
**payroll_run** | Option<**String**> | The payroll being run. | [optional]
**gross_pay** | Option<**f32**> | The gross pay from the run. | [optional]
**net_pay** | Option<**f32**> | The net pay from the run. | [optional]
**start_date** | Option<**String**> | The day and time the payroll run started. | [optional]
**end_date** | Option<**String**> | The day and time the payroll run ended. | [optional]
**check_date** | Option<**String**> | The day and time the payroll run was checked. | [optional]
**earnings** | Option<[**Vec<crate::models::Earning>**](Earning.md)> |  | [optional][readonly]
**deductions** | Option<[**Vec<crate::models::Deduction>**](Deduction.md)> |  | [optional][readonly]
**taxes** | Option<[**Vec<crate::models::Tax>**](Tax.md)> |  | [optional][readonly]
**remote_data** | Option<[**Vec<crate::models::RemoteData>**](RemoteData.md)> |  | [optional][readonly]
**remote_was_deleted** | Option<**bool**> | Indicates whether or not this object has been deleted by third party webhooks. | [optional][readonly]
**field_mappings** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



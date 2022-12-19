# Deduction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**employee_payroll_run** | Option<**String**> | The deduction's employee payroll run. | [optional]
**name** | Option<**String**> | The deduction's name. | [optional]
**employee_deduction** | Option<**f32**> | The amount the employee is deducting. | [optional]
**company_deduction** | Option<**f32**> | The amount the company is deducting. | [optional]
**remote_data** | Option<[**Vec<crate::models::RemoteData>**](RemoteData.md)> |  | [optional][readonly]
**remote_was_deleted** | Option<**bool**> | Indicates whether or not this object has been deleted by third party webhooks. | [optional]
**field_mappings** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



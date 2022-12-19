# TimeOffRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**employee** | Option<**String**> | The employee requesting time off. | [optional]
**approver** | Option<**String**> | The employee approving the time off request. | [optional]
**status** | Option<[**crate::models::TimeOffStatusEnum**](TimeOffStatusEnum.md)> | The status of this time off request. | [optional]
**employee_note** | Option<**String**> | The employee note for this time off request. | [optional]
**units** | Option<[**crate::models::UnitsEnum**](UnitsEnum.md)> | The unit of time requested. | [optional]
**amount** | Option<**f32**> | The number of time off units requested. | [optional]
**request_type** | Option<[**crate::models::RequestTypeEnum**](RequestTypeEnum.md)> | The type of time off request. | [optional]
**start_time** | Option<**String**> | The day and time of the start of the time requested off. | [optional]
**end_time** | Option<**String**> | The day and time of the end of the time requested off. | [optional]
**integration_params** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**linked_account_params** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



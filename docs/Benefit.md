# Benefit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**employee** | Option<**String**> | The employee on the plan. | [optional]
**provider_name** | Option<**String**> | The name of the benefit provider. | [optional]
**benefit_plan_type** | Option<**String**> | The type of benefit plan | [optional]
**employee_contribution** | Option<**f32**> | The employee's contribution. | [optional]
**company_contribution** | Option<**f32**> | The company's contribution. | [optional]
**remote_data** | Option<[**Vec<crate::models::RemoteData>**](RemoteData.md)> |  | [optional][readonly]
**remote_was_deleted** | Option<**bool**> | Indicates whether or not this object has been deleted by third party webhooks. | [optional][readonly]
**field_mappings** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



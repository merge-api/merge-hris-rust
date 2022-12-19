# LinkedAccountCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition_schema_id** | **String** | The ID indicating which condition schema to use for a specific condition. | 
**common_model** | Option<**String**> | The common model for a specific condition. | [optional][readonly]
**native_name** | Option<**String**> | User-facing *native condition* name. e.g. \"Skip Manager\". | 
**operator** | **String** | The operator for a specific condition. | 
**value** | Option<[**serde_json::Value**](.md)> | The value for a condition. | [optional][readonly]
**field_name** | Option<**String**> | The name of the field on the common model that this condition corresponds to, if they conceptually match. e.g. \"location_type\". | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



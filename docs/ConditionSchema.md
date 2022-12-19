# ConditionSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the condition schema. This ID is used when updating selective syncs for a linked account. | 
**common_model** | Option<**String**> | The common model for which a condition schema is defined. | [optional][readonly]
**native_name** | Option<**String**> | User-facing *native condition* name. e.g. \"Skip Manager\". | 
**field_name** | Option<**String**> | The name of the field on the common model that this condition corresponds to, if they conceptually match. e.g. \"location_type\". | 
**is_unique** | Option<**bool**> | Whether this condition can only be applied once. If false, the condition can be AND'd together multiple times. | [optional]
**condition_type** | [**crate::models::ConditionTypeEnum**](ConditionTypeEnum.md) |  | 
**operators** | [**Vec<crate::models::OperatorSchema>**](OperatorSchema.md) | The schemas for the operators that can be used on a condition. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



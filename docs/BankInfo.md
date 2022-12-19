# BankInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**employee** | Option<**String**> | The employee with this bank account. | [optional]
**account_number** | Option<**String**> | The account number. | [optional]
**routing_number** | Option<**String**> | The routing number. | [optional]
**bank_name** | Option<**String**> | The bank name. | [optional]
**account_type** | Option<[**crate::models::AccountTypeEnum**](AccountTypeEnum.md)> | The bank account type | [optional]
**remote_created_at** | Option<**String**> | When the matching bank object was created in the third party system. | [optional]
**remote_data** | Option<[**Vec<crate::models::RemoteData>**](RemoteData.md)> |  | [optional][readonly]
**remote_was_deleted** | Option<**bool**> | Indicates whether or not this object has been deleted by third party webhooks. | [optional][readonly]
**field_mappings** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



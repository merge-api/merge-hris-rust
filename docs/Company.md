# Company

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**legal_name** | Option<**String**> | The company's legal name. | [optional]
**display_name** | Option<**String**> | The company's display name. | [optional]
**eins** | Option<**Vec<String>**> | The company's Employer Identification Numbers. | [optional]
**remote_data** | Option<[**Vec<crate::models::RemoteData>**](RemoteData.md)> |  | [optional][readonly]
**remote_was_deleted** | Option<**bool**> | Indicates whether or not this object has been deleted by third party webhooks. | [optional][readonly]
**field_mappings** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



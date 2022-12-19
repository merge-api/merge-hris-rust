# Location

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**name** | Option<**String**> | The location's name. | [optional]
**phone_number** | Option<**String**> | The location's phone number. | [optional]
**street_1** | Option<**String**> | Line 1 of the location's street address. | [optional]
**street_2** | Option<**String**> | Line 2 of the location's street address. | [optional]
**city** | Option<**String**> | The location's city. | [optional]
**state** | Option<**String**> | The location's state. Represents a region if outside of the US. | [optional]
**zip_code** | Option<**String**> | The location's zip code or postal code. | [optional]
**country** | Option<[**crate::models::CountryEnum**](CountryEnum.md)> | The location's country. | [optional]
**location_type** | Option<[**crate::models::LocationTypeEnum**](LocationTypeEnum.md)> | The location's type. Can be either WORK or HOME | [optional]
**remote_data** | Option<[**Vec<crate::models::RemoteData>**](RemoteData.md)> |  | [optional][readonly]
**remote_was_deleted** | Option<**bool**> | Indicates whether or not this object has been deleted by third party webhooks. | [optional][readonly]
**field_mappings** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



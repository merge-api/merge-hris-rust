# AccountDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**integration** | Option<**String**> |  | [optional][readonly]
**integration_slug** | Option<**String**> |  | [optional][readonly]
**category** | Option<[**crate::models::CategoryEnum**](CategoryEnum.md)> |  | [optional]
**end_user_origin_id** | Option<**String**> |  | [optional][readonly]
**end_user_organization_name** | Option<**String**> |  | [optional][readonly]
**end_user_email_address** | Option<**String**> |  | [optional][readonly]
**status** | Option<**String**> |  | [optional][readonly]
**webhook_listener_url** | Option<**String**> |  | [optional][readonly]
**is_duplicate** | Option<**bool**> | Whether a Production Linked Account's credentials match another existing Production Linked Account. This field is `null` for Test Linked Accounts, incomplete Production Linked Accounts, and ignored duplicate Production Linked Account sets. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



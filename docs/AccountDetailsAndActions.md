# AccountDetailsAndActions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**category** | Option<[**crate::models::CategoryEnum**](CategoryEnum.md)> |  | [optional]
**status** | Option<[**crate::models::AccountDetailsAndActionsStatusEnum**](AccountDetailsAndActionsStatusEnum.md)> |  | 
**status_detail** | Option<**String**> |  | [optional]
**end_user_origin_id** | Option<**String**> |  | [optional]
**end_user_organization_name** | **String** |  | 
**end_user_email_address** | **String** |  | 
**webhook_listener_url** | **String** |  | 
**is_duplicate** | Option<**bool**> | Whether a Production Linked Account's credentials match another existing Production Linked Account. This field is `null` for Test Linked Accounts, incomplete Production Linked Accounts, and ignored duplicate Production Linked Account sets. | [optional]
**integration** | Option<[**crate::models::AccountDetailsAndActionsIntegration**](AccountDetailsAndActionsIntegration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



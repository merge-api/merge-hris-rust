# \LinkedAccountsApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**linked_accounts_list**](LinkedAccountsApi.md#linked_accounts_list) | **GET** /linked-accounts | 



## linked_accounts_list

> crate::models::PaginatedAccountDetailsAndActionsList linked_accounts_list(category, cursor, end_user_email_address, end_user_organization_name, end_user_origin_id, end_user_origin_ids, id, ids, include_duplicates, integration_name, is_test_account, page_size, status)


List linked accounts for your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | Option<**String**> |  |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**end_user_email_address** | Option<**String**> | If provided, will only return linked accounts associated with the given email address. |  |
**end_user_organization_name** | Option<**String**> | If provided, will only return linked accounts associated with the given organization name. |  |
**end_user_origin_id** | Option<**String**> | If provided, will only return linked accounts associated with the given origin ID. |  |
**end_user_origin_ids** | Option<**String**> | Comma-separated list of EndUser origin IDs, making it possible to specify multiple EndUsers at once. |  |
**id** | Option<**String**> |  |  |
**ids** | Option<**String**> | Comma-separated list of LinkedAccount IDs, making it possible to specify multiple LinkedAccounts at once. |  |
**include_duplicates** | Option<**bool**> | If `true`, will include complete production duplicates of the account specified by the `id` query parameter in the response. `id` must be for a complete production linked account. |  |
**integration_name** | Option<**String**> | If provided, will only return linked accounts associated with the given integration name. |  |
**is_test_account** | Option<**String**> | If included, will only include test linked accounts. If not included, will only include non-test linked accounts. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**status** | Option<**String**> | Filter by status. Options: `COMPLETE`, `INCOMPLETE`, `RELINK_NEEDED` |  |

### Return type

[**crate::models::PaginatedAccountDetailsAndActionsList**](PaginatedAccountDetailsAndActionsList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


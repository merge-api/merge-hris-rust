# \SyncStatusApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sync_status_list**](SyncStatusApi.md#sync_status_list) | **GET** /sync-status | 



## sync_status_list

> crate::models::PaginatedSyncStatusList sync_status_list(x_account_token, cursor, page_size)


Get syncing status. Possible values: `DISABLED`, `DONE`, `FAILED`, `PAUSED`, `SYNCING`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |

### Return type

[**crate::models::PaginatedSyncStatusList**](PaginatedSyncStatusList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


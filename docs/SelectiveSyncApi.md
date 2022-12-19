# \SelectiveSyncApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**selective_sync_configurations_list**](SelectiveSyncApi.md#selective_sync_configurations_list) | **GET** /selective-sync/configurations | 
[**selective_sync_configurations_update**](SelectiveSyncApi.md#selective_sync_configurations_update) | **PUT** /selective-sync/configurations | 
[**selective_sync_meta_list**](SelectiveSyncApi.md#selective_sync_meta_list) | **GET** /selective-sync/meta | 



## selective_sync_configurations_list

> Vec<crate::models::LinkedAccountSelectiveSyncConfiguration> selective_sync_configurations_list(x_account_token)


Get a linked account's selective syncs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |

### Return type

[**Vec<crate::models::LinkedAccountSelectiveSyncConfiguration>**](LinkedAccountSelectiveSyncConfiguration.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selective_sync_configurations_update

> Vec<crate::models::LinkedAccountSelectiveSyncConfiguration> selective_sync_configurations_update(x_account_token, linked_account_selective_sync_configuration_list_request)


Replace a linked account's selective syncs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**linked_account_selective_sync_configuration_list_request** | [**LinkedAccountSelectiveSyncConfigurationListRequest**](LinkedAccountSelectiveSyncConfigurationListRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::LinkedAccountSelectiveSyncConfiguration>**](LinkedAccountSelectiveSyncConfiguration.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## selective_sync_meta_list

> crate::models::PaginatedConditionSchemaList selective_sync_meta_list(x_account_token, common_model, cursor, page_size)


Get metadata for the conditions available to a linked account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**common_model** | Option<**String**> |  |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |

### Return type

[**crate::models::PaginatedConditionSchemaList**](PaginatedConditionSchemaList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


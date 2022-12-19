# \PassthroughApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**passthrough_create**](PassthroughApi.md#passthrough_create) | **POST** /passthrough | 



## passthrough_create

> crate::models::RemoteResponse passthrough_create(x_account_token, data_passthrough_request)


Pull data from an endpoint not currently supported by Merge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**data_passthrough_request** | [**DataPassthroughRequest**](DataPassthroughRequest.md) |  | [required] |

### Return type

[**crate::models::RemoteResponse**](RemoteResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


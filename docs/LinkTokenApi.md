# \LinkTokenApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**link_token_create**](LinkTokenApi.md#link_token_create) | **POST** /link-token | 



## link_token_create

> crate::models::LinkToken link_token_create(end_user_details_request)


Creates a link token to be used when linking a new end user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**end_user_details_request** | [**EndUserDetailsRequest**](EndUserDetailsRequest.md) |  | [required] |

### Return type

[**crate::models::LinkToken**](LinkToken.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


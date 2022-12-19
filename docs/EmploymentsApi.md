# \EmploymentsApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**employments_list**](EmploymentsApi.md#employments_list) | **GET** /employments | 
[**employments_retrieve**](EmploymentsApi.md#employments_retrieve) | **GET** /employments/{id} | 



## employments_list

> crate::models::PaginatedEmploymentList employments_list(x_account_token, created_after, created_before, cursor, employee_id, include_deleted_data, include_remote_data, modified_after, modified_before, order_by, page_size, remote_fields, remote_id)


Returns a list of `Employment` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**created_after** | Option<**String**> | If provided, will only return objects created after this datetime. |  |
**created_before** | Option<**String**> | If provided, will only return objects created before this datetime. |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**employee_id** | Option<**String**> | If provided, will only return employments for this employee. |  |
**include_deleted_data** | Option<**bool**> | Whether to include data that was marked as deleted by third party webhooks. |  |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**modified_after** | Option<**String**> | If provided, will only return objects modified after this datetime. |  |
**modified_before** | Option<**String**> | If provided, will only return objects modified before this datetime. |  |
**order_by** | Option<**String**> | Overrides the default ordering for this endpoint. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |
**remote_id** | Option<**String**> | The API provider's ID for the given object. |  |

### Return type

[**crate::models::PaginatedEmploymentList**](PaginatedEmploymentList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employments_retrieve

> crate::models::Employment employments_retrieve(x_account_token, id, include_remote_data, remote_fields)


Returns an `Employment` object with the given `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**id** | **String** |  | [required] |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |

### Return type

[**crate::models::Employment**](Employment.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


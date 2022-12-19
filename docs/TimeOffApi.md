# \TimeOffApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**time_off_create**](TimeOffApi.md#time_off_create) | **POST** /time-off | 
[**time_off_list**](TimeOffApi.md#time_off_list) | **GET** /time-off | 
[**time_off_meta_post_retrieve**](TimeOffApi.md#time_off_meta_post_retrieve) | **GET** /time-off/meta/post | 
[**time_off_retrieve**](TimeOffApi.md#time_off_retrieve) | **GET** /time-off/{id} | 



## time_off_create

> crate::models::TimeOffResponse time_off_create(x_account_token, time_off_endpoint_request, is_debug_mode, run_async)


Creates a `TimeOff` object with the given values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**time_off_endpoint_request** | [**TimeOffEndpointRequest**](TimeOffEndpointRequest.md) |  | [required] |
**is_debug_mode** | Option<**bool**> | Whether to include debug fields (such as log file links) in the response. |  |
**run_async** | Option<**bool**> | Whether or not third-party updates should be run asynchronously. |  |

### Return type

[**crate::models::TimeOffResponse**](TimeOffResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_off_list

> crate::models::PaginatedTimeOffList time_off_list(x_account_token, approver_id, created_after, created_before, cursor, employee_id, include_deleted_data, include_remote_data, modified_after, modified_before, page_size, remote_fields, remote_id, request_type, status)


Returns a list of `TimeOff` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**approver_id** | Option<**String**> | If provided, will only return time off for this approver. |  |
**created_after** | Option<**String**> | If provided, will only return objects created after this datetime. |  |
**created_before** | Option<**String**> | If provided, will only return objects created before this datetime. |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**employee_id** | Option<**String**> | If provided, will only return time off for this employee. |  |
**include_deleted_data** | Option<**bool**> | Whether to include data that was marked as deleted by third party webhooks. |  |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**modified_after** | Option<**String**> | If provided, will only return objects modified after this datetime. |  |
**modified_before** | Option<**String**> | If provided, will only return objects modified before this datetime. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |
**remote_id** | Option<**String**> | The API provider's ID for the given object. |  |
**request_type** | Option<**String**> | If provided, will only return TimeOff with this request type. Options: ('VACATION', 'SICK', 'PERSONAL', 'JURY_DUTY', 'VOLUNTEER', 'BEREAVEMENT') |  |
**status** | Option<**String**> | If provided, will only return TimeOff with this status. Options: ('REQUESTED', 'APPROVED', 'DECLINED', 'CANCELLED', 'DELETED') |  |

### Return type

[**crate::models::PaginatedTimeOffList**](PaginatedTimeOffList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_off_meta_post_retrieve

> crate::models::MetaResponse time_off_meta_post_retrieve(x_account_token)


Returns metadata for `TimeOff` POSTs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |

### Return type

[**crate::models::MetaResponse**](MetaResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## time_off_retrieve

> crate::models::TimeOff time_off_retrieve(x_account_token, id, include_remote_data, remote_fields)


Returns a `TimeOff` object with the given `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**id** | **String** |  | [required] |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |

### Return type

[**crate::models::TimeOff**](TimeOff.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


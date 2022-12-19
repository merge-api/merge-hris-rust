# \EmployeesApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**employees_create**](EmployeesApi.md#employees_create) | **POST** /employees | 
[**employees_ignore_create**](EmployeesApi.md#employees_ignore_create) | **POST** /employees/ignore/{model_id} | 
[**employees_list**](EmployeesApi.md#employees_list) | **GET** /employees | 
[**employees_meta_post_retrieve**](EmployeesApi.md#employees_meta_post_retrieve) | **GET** /employees/meta/post | 
[**employees_retrieve**](EmployeesApi.md#employees_retrieve) | **GET** /employees/{id} | 



## employees_create

> crate::models::EmployeeResponse employees_create(x_account_token, employee_endpoint_request, is_debug_mode, run_async)


Creates an `Employee` object with the given values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**employee_endpoint_request** | [**EmployeeEndpointRequest**](EmployeeEndpointRequest.md) |  | [required] |
**is_debug_mode** | Option<**bool**> | Whether to include debug fields (such as log file links) in the response. |  |
**run_async** | Option<**bool**> | Whether or not third-party updates should be run asynchronously. |  |

### Return type

[**crate::models::EmployeeResponse**](EmployeeResponse.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employees_ignore_create

> crate::models::IgnoreCommonModel employees_ignore_create(x_account_token, model_id, ignore_common_model_request)


Ignores a specific row based on the `model_id` in the url. These records will have their properties set to null, and will not be updated in future syncs. The \"reason\" and \"message\" fields in the request body will be stored for audit purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**model_id** | **String** |  | [required] |
**ignore_common_model_request** | [**IgnoreCommonModelRequest**](IgnoreCommonModelRequest.md) |  | [required] |

### Return type

[**crate::models::IgnoreCommonModel**](IgnoreCommonModel.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employees_list

> crate::models::PaginatedEmployeeList employees_list(x_account_token, company_id, created_after, created_before, cursor, display_full_name, employment_status, first_name, groups, include_deleted_data, include_remote_data, include_sensitive_fields, last_name, manager_id, modified_after, modified_before, page_size, pay_group_id, personal_email, remote_fields, remote_id, team_id, work_email, work_location_id)


Returns a list of `Employee` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**company_id** | Option<**String**> | If provided, will only return employees for this company. |  |
**created_after** | Option<**String**> | If provided, will only return objects created after this datetime. |  |
**created_before** | Option<**String**> | If provided, will only return objects created before this datetime. |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**display_full_name** | Option<**String**> | If provided, will only return employees with this display name. |  |
**employment_status** | Option<**String**> | If provided, will only return employees with this employment status. |  |
**first_name** | Option<**String**> | If provided, will only return employees with this first name. |  |
**groups** | Option<**String**> | If provided, will only return employees matching the group ids; multiple groups can be separated by commas. |  |
**include_deleted_data** | Option<**bool**> | Whether to include data that was marked as deleted by third party webhooks. |  |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**include_sensitive_fields** | Option<**bool**> | Whether to include sensitive fields (such as social security numbers) in the response. |  |
**last_name** | Option<**String**> | If provided, will only return employees with this last name. |  |
**manager_id** | Option<**String**> | If provided, will only return employees for this manager. |  |
**modified_after** | Option<**String**> | If provided, will only return objects modified after this datetime. |  |
**modified_before** | Option<**String**> | If provided, will only return objects modified before this datetime. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**pay_group_id** | Option<**String**> | If provided, will only return employees for this pay group |  |
**personal_email** | Option<**String**> | If provided, will only return Employees with this personal email |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |
**remote_id** | Option<**String**> | The API provider's ID for the given object. |  |
**team_id** | Option<**String**> | If provided, will only return employees for this team. |  |
**work_email** | Option<**String**> | If provided, will only return Employees with this work email |  |
**work_location_id** | Option<**String**> | If provided, will only return employees for this location. |  |

### Return type

[**crate::models::PaginatedEmployeeList**](PaginatedEmployeeList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employees_meta_post_retrieve

> crate::models::MetaResponse employees_meta_post_retrieve(x_account_token)


Returns metadata for `Employee` POSTs.

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


## employees_retrieve

> crate::models::Employee employees_retrieve(x_account_token, id, include_remote_data, include_sensitive_fields, remote_fields)


Returns an `Employee` object with the given `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**id** | **String** |  | [required] |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**include_sensitive_fields** | Option<**bool**> | Whether to include sensitive fields (such as social security numbers) in the response. |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |

### Return type

[**crate::models::Employee**](Employee.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \PayrollRunsApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payroll_runs_list**](PayrollRunsApi.md#payroll_runs_list) | **GET** /payroll-runs | 
[**payroll_runs_retrieve**](PayrollRunsApi.md#payroll_runs_retrieve) | **GET** /payroll-runs/{id} | 



## payroll_runs_list

> crate::models::PaginatedPayrollRunList payroll_runs_list(x_account_token, created_after, created_before, cursor, ended_after, ended_before, include_deleted_data, include_remote_data, modified_after, modified_before, page_size, remote_fields, remote_id, run_type, started_after, started_before)


Returns a list of `PayrollRun` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**created_after** | Option<**String**> | If provided, will only return objects created after this datetime. |  |
**created_before** | Option<**String**> | If provided, will only return objects created before this datetime. |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**ended_after** | Option<**String**> | If provided, will only return payroll runs ended after this datetime. |  |
**ended_before** | Option<**String**> | If provided, will only return payroll runs ended before this datetime. |  |
**include_deleted_data** | Option<**bool**> | Whether to include data that was marked as deleted by third party webhooks. |  |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**modified_after** | Option<**String**> | If provided, will only return objects modified after this datetime. |  |
**modified_before** | Option<**String**> | If provided, will only return objects modified before this datetime. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |
**remote_id** | Option<**String**> | The API provider's ID for the given object. |  |
**run_type** | Option<**String**> | If provided, will only return PayrollRun's with this status. Options: ('REGULAR', 'OFF_CYCLE', 'CORRECTION', 'TERMINATION', 'SIGN_ON_BONUS') |  |
**started_after** | Option<**String**> | If provided, will only return payroll runs started after this datetime. |  |
**started_before** | Option<**String**> | If provided, will only return payroll runs started before this datetime. |  |

### Return type

[**crate::models::PaginatedPayrollRunList**](PaginatedPayrollRunList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payroll_runs_retrieve

> crate::models::PayrollRun payroll_runs_retrieve(x_account_token, id, include_remote_data, remote_fields)


Returns a `PayrollRun` object with the given `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**id** | **String** |  | [required] |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**remote_fields** | Option<**String**> | Which fields should be returned in non-normalized form. |  |

### Return type

[**crate::models::PayrollRun**](PayrollRun.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


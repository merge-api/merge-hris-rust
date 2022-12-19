# \EmployeePayrollRunsApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**employee_payroll_runs_list**](EmployeePayrollRunsApi.md#employee_payroll_runs_list) | **GET** /employee-payroll-runs | 
[**employee_payroll_runs_retrieve**](EmployeePayrollRunsApi.md#employee_payroll_runs_retrieve) | **GET** /employee-payroll-runs/{id} | 



## employee_payroll_runs_list

> crate::models::PaginatedEmployeePayrollRunList employee_payroll_runs_list(x_account_token, created_after, created_before, cursor, employee_id, ended_after, ended_before, include_deleted_data, include_remote_data, modified_after, modified_before, page_size, payroll_run_id, remote_id, started_after, started_before)


Returns a list of `EmployeePayrollRun` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**created_after** | Option<**String**> | If provided, will only return objects created after this datetime. |  |
**created_before** | Option<**String**> | If provided, will only return objects created before this datetime. |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**employee_id** | Option<**String**> | If provided, will only return employee payroll runs for this employee. |  |
**ended_after** | Option<**String**> | If provided, will only return employee payroll runs ended after this datetime. |  |
**ended_before** | Option<**String**> | If provided, will only return employee payroll runs ended before this datetime. |  |
**include_deleted_data** | Option<**bool**> | Whether to include data that was marked as deleted by third party webhooks. |  |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**modified_after** | Option<**String**> | If provided, will only return objects modified after this datetime. |  |
**modified_before** | Option<**String**> | If provided, will only return objects modified before this datetime. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**payroll_run_id** | Option<**String**> | If provided, will only return employee payroll runs for this employee. |  |
**remote_id** | Option<**String**> | The API provider's ID for the given object. |  |
**started_after** | Option<**String**> | If provided, will only return employee payroll runs started after this datetime. |  |
**started_before** | Option<**String**> | If provided, will only return employee payroll runs started before this datetime. |  |

### Return type

[**crate::models::PaginatedEmployeePayrollRunList**](PaginatedEmployeePayrollRunList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## employee_payroll_runs_retrieve

> crate::models::EmployeePayrollRun employee_payroll_runs_retrieve(x_account_token, id, include_remote_data)


Returns an `EmployeePayrollRun` object with the given `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**id** | **String** |  | [required] |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |

### Return type

[**crate::models::EmployeePayrollRun**](EmployeePayrollRun.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


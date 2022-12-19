# \IssuesApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**issues_list**](IssuesApi.md#issues_list) | **GET** /issues | 
[**issues_retrieve**](IssuesApi.md#issues_retrieve) | **GET** /issues/{id} | 



## issues_list

> crate::models::PaginatedIssueList issues_list(account_token, cursor, end_date, end_user_organization_name, first_incident_time_after, first_incident_time_before, include_muted, integration_name, last_incident_time_after, last_incident_time_before, page_size, start_date, status)


Gets issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_token** | Option<**String**> |  |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**end_date** | Option<**String**> | If included, will only include issues whose most recent action occurred before this time |  |
**end_user_organization_name** | Option<**String**> |  |  |
**first_incident_time_after** | Option<**String**> | If provided, will only return issues whose first incident time was after this datetime. |  |
**first_incident_time_before** | Option<**String**> | If provided, will only return issues whose first incident time was before this datetime. |  |
**include_muted** | Option<**String**> | If True, will include muted issues |  |
**integration_name** | Option<**String**> |  |  |
**last_incident_time_after** | Option<**String**> | If provided, will only return issues whose first incident time was after this datetime. |  |
**last_incident_time_before** | Option<**String**> | If provided, will only return issues whose first incident time was before this datetime. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**start_date** | Option<**String**> | If included, will only include issues whose most recent action occurred after this time |  |
**status** | Option<**String**> |  |  |

### Return type

[**crate::models::PaginatedIssueList**](PaginatedIssueList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issues_retrieve

> crate::models::Issue issues_retrieve(id)


Get a specific issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Issue**](Issue.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


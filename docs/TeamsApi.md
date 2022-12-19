# \TeamsApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**teams_list**](TeamsApi.md#teams_list) | **GET** /teams | 
[**teams_retrieve**](TeamsApi.md#teams_retrieve) | **GET** /teams/{id} | 



## teams_list

> crate::models::PaginatedTeamList teams_list(x_account_token, created_after, created_before, cursor, include_deleted_data, include_remote_data, modified_after, modified_before, page_size, parent_team_id, remote_id)


Returns a list of `Team` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**created_after** | Option<**String**> | If provided, will only return objects created after this datetime. |  |
**created_before** | Option<**String**> | If provided, will only return objects created before this datetime. |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**include_deleted_data** | Option<**bool**> | Whether to include data that was marked as deleted by third party webhooks. |  |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**modified_after** | Option<**String**> | If provided, will only return objects modified after this datetime. |  |
**modified_before** | Option<**String**> | If provided, will only return objects modified before this datetime. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**parent_team_id** | Option<**String**> | If provided, will only return teams with this parent team. |  |
**remote_id** | Option<**String**> | The API provider's ID for the given object. |  |

### Return type

[**crate::models::PaginatedTeamList**](PaginatedTeamList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_retrieve

> crate::models::Team teams_retrieve(x_account_token, id, include_remote_data)


Returns a `Team` object with the given `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**id** | **String** |  | [required] |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |

### Return type

[**crate::models::Team**](Team.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


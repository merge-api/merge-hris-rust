# \BenefitsApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**benefits_list**](BenefitsApi.md#benefits_list) | **GET** /benefits | 
[**benefits_retrieve**](BenefitsApi.md#benefits_retrieve) | **GET** /benefits/{id} | 



## benefits_list

> crate::models::PaginatedBenefitList benefits_list(x_account_token, created_after, created_before, cursor, employee_id, include_deleted_data, include_remote_data, modified_after, modified_before, page_size, remote_id)


Returns a list of `Benefit` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**created_after** | Option<**String**> | If provided, will only return objects created after this datetime. |  |
**created_before** | Option<**String**> | If provided, will only return objects created before this datetime. |  |
**cursor** | Option<**String**> | The pagination cursor value. |  |
**employee_id** | Option<**String**> | If provided, will return the benefits associated with the employee. |  |
**include_deleted_data** | Option<**bool**> | Whether to include data that was marked as deleted by third party webhooks. |  |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |
**modified_after** | Option<**String**> | If provided, will only return objects modified after this datetime. |  |
**modified_before** | Option<**String**> | If provided, will only return objects modified before this datetime. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**remote_id** | Option<**String**> | The API provider's ID for the given object. |  |

### Return type

[**crate::models::PaginatedBenefitList**](PaginatedBenefitList.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## benefits_retrieve

> crate::models::Benefit benefits_retrieve(x_account_token, id, include_remote_data)


Returns a `Benefit` object with the given `id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**id** | **String** |  | [required] |
**include_remote_data** | Option<**bool**> | Whether to include the original data Merge fetched from the third-party to produce these models. |  |

### Return type

[**crate::models::Benefit**](Benefit.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


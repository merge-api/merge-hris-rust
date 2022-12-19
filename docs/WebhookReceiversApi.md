# \WebhookReceiversApi

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhook_receivers_create**](WebhookReceiversApi.md#webhook_receivers_create) | **POST** /webhook-receivers | 
[**webhook_receivers_list**](WebhookReceiversApi.md#webhook_receivers_list) | **GET** /webhook-receivers | 



## webhook_receivers_create

> crate::models::WebhookReceiver webhook_receivers_create(x_account_token, webhook_receiver_request)


Creates a `WebhookReceiver` object with the given values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |
**webhook_receiver_request** | [**WebhookReceiverRequest**](WebhookReceiverRequest.md) |  | [required] |

### Return type

[**crate::models::WebhookReceiver**](WebhookReceiver.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_receivers_list

> Vec<crate::models::WebhookReceiver> webhook_receivers_list(x_account_token)


Returns a list of `WebhookReceiver` objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_account_token** | **String** | Token identifying the end user. | [required] |

### Return type

[**Vec<crate::models::WebhookReceiver>**](WebhookReceiver.md)

### Authorization

[tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


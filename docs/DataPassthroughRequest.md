# DataPassthroughRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method** | Option<[**crate::models::MethodEnum**](MethodEnum.md)> |  | 
**path** | **String** |  | 
**base_url_override** | Option<**String**> |  | [optional]
**data** | Option<**String**> |  | [optional]
**multipart_form_data** | Option<[**Vec<crate::models::MultipartFormFieldRequest>**](MultipartFormFieldRequest.md)> | Pass an array of `MultipartFormField` objects in here instead of using the `data` param if `request_format` is set to `MULTIPART`. | [optional]
**headers** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The headers to use for the request (Merge will handle the account's authorization headers). `Content-Type` header is required for passthrough. Choose content type corresponding to expected format of receiving server. | [optional]
**request_format** | Option<[**crate::models::RequestFormatEnum**](RequestFormatEnum.md)> |  | [optional]
**normalize_response** | Option<**bool**> | Optional. If true, the response will always be an object of the form `{\"type\": T, \"value\": ...}` where `T` will be one of `string, boolean, number, null, array, object`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



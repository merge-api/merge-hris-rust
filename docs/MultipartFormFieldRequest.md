# MultipartFormFieldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the form field | 
**data** | **String** | The data for the form field. | 
**encoding** | Option<[**crate::models::EncodingEnum**](EncodingEnum.md)> | The encoding of the value of `data`. Defaults to `RAW` if not defined. | [optional]
**file_name** | Option<**String**> | The file name of the form field, if the field is for a file. | [optional]
**content_type** | Option<**String**> | The MIME type of the file, if the field is for a file. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



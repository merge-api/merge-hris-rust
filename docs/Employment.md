# Employment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional][readonly]
**remote_id** | Option<**String**> | The third-party API ID of the matching object. | [optional]
**employee** | Option<**String**> | The employee holding this position. | [optional]
**job_title** | Option<**String**> | The position's title. | [optional]
**pay_rate** | Option<**f32**> | The position's pay rate in dollars. | [optional]
**pay_period** | Option<[**crate::models::PayPeriodEnum**](PayPeriodEnum.md)> | The time period this pay rate encompasses. | [optional]
**pay_frequency** | Option<[**crate::models::PayFrequencyEnum**](PayFrequencyEnum.md)> | The position's pay frequency. | [optional]
**pay_currency** | Option<[**crate::models::PayCurrencyEnum**](PayCurrencyEnum.md)> | The position's currency code. | [optional]
**pay_group** | Option<**String**> | The employment's pay group | [optional]
**flsa_status** | Option<[**crate::models::FlsaStatusEnum**](FlsaStatusEnum.md)> | The position's FLSA status. | [optional]
**effective_date** | Option<**String**> | The position's effective date. | [optional]
**employment_type** | Option<[**crate::models::EmploymentTypeEnum**](EmploymentTypeEnum.md)> | The position's type of employment. | [optional]
**remote_data** | Option<[**Vec<crate::models::RemoteData>**](RemoteData.md)> |  | [optional][readonly]
**remote_was_deleted** | Option<**bool**> | Indicates whether or not this object has been deleted by third party webhooks. | [optional][readonly]
**field_mappings** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# Rust API client for @mergeapi/merge-hris-rust

The unified API for building rich integrations with multiple HR Information System platforms.

For more information, please visit [https://www.merge.dev/](https://www.merge.dev/)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 0.1.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `@mergeapi/merge-hris-rust` and add the following to `Cargo.toml` under `[dependencies]`:

```
@mergeapi/merge-hris-rust = { path = "./@mergeapi/merge-hris-rust" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.merge.dev/api/hris/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountDetailsApi* | [**account_details_retrieve**](docs/AccountDetailsApi.md#account_details_retrieve) | **GET** /account-details | 
*AccountTokenApi* | [**account_token_retrieve**](docs/AccountTokenApi.md#account_token_retrieve) | **GET** /account-token/{public_token} | 
*AvailableActionsApi* | [**available_actions_retrieve**](docs/AvailableActionsApi.md#available_actions_retrieve) | **GET** /available-actions | 
*BankInfoApi* | [**bank_info_list**](docs/BankInfoApi.md#bank_info_list) | **GET** /bank-info | 
*BankInfoApi* | [**bank_info_retrieve**](docs/BankInfoApi.md#bank_info_retrieve) | **GET** /bank-info/{id} | 
*BenefitsApi* | [**benefits_list**](docs/BenefitsApi.md#benefits_list) | **GET** /benefits | 
*BenefitsApi* | [**benefits_retrieve**](docs/BenefitsApi.md#benefits_retrieve) | **GET** /benefits/{id} | 
*CompaniesApi* | [**companies_list**](docs/CompaniesApi.md#companies_list) | **GET** /companies | 
*CompaniesApi* | [**companies_retrieve**](docs/CompaniesApi.md#companies_retrieve) | **GET** /companies/{id} | 
*DeductionsApi* | [**deductions_list**](docs/DeductionsApi.md#deductions_list) | **GET** /deductions | 
*DeductionsApi* | [**deductions_retrieve**](docs/DeductionsApi.md#deductions_retrieve) | **GET** /deductions/{id} | 
*DeleteAccountApi* | [**delete_account_create**](docs/DeleteAccountApi.md#delete_account_create) | **POST** /delete-account | 
*EmployeePayrollRunsApi* | [**employee_payroll_runs_list**](docs/EmployeePayrollRunsApi.md#employee_payroll_runs_list) | **GET** /employee-payroll-runs | 
*EmployeePayrollRunsApi* | [**employee_payroll_runs_retrieve**](docs/EmployeePayrollRunsApi.md#employee_payroll_runs_retrieve) | **GET** /employee-payroll-runs/{id} | 
*EmployeesApi* | [**employees_create**](docs/EmployeesApi.md#employees_create) | **POST** /employees | 
*EmployeesApi* | [**employees_ignore_create**](docs/EmployeesApi.md#employees_ignore_create) | **POST** /employees/ignore/{model_id} | 
*EmployeesApi* | [**employees_list**](docs/EmployeesApi.md#employees_list) | **GET** /employees | 
*EmployeesApi* | [**employees_meta_post_retrieve**](docs/EmployeesApi.md#employees_meta_post_retrieve) | **GET** /employees/meta/post | 
*EmployeesApi* | [**employees_retrieve**](docs/EmployeesApi.md#employees_retrieve) | **GET** /employees/{id} | 
*EmploymentsApi* | [**employments_list**](docs/EmploymentsApi.md#employments_list) | **GET** /employments | 
*EmploymentsApi* | [**employments_retrieve**](docs/EmploymentsApi.md#employments_retrieve) | **GET** /employments/{id} | 
*ForceResyncApi* | [**sync_status_resync_create**](docs/ForceResyncApi.md#sync_status_resync_create) | **POST** /sync-status/resync | 
*GenerateKeyApi* | [**generate_key_create**](docs/GenerateKeyApi.md#generate_key_create) | **POST** /generate-key | 
*GroupsApi* | [**groups_list**](docs/GroupsApi.md#groups_list) | **GET** /groups | 
*GroupsApi* | [**groups_retrieve**](docs/GroupsApi.md#groups_retrieve) | **GET** /groups/{id} | 
*IssuesApi* | [**issues_list**](docs/IssuesApi.md#issues_list) | **GET** /issues | 
*IssuesApi* | [**issues_retrieve**](docs/IssuesApi.md#issues_retrieve) | **GET** /issues/{id} | 
*LinkTokenApi* | [**link_token_create**](docs/LinkTokenApi.md#link_token_create) | **POST** /link-token | 
*LinkedAccountsApi* | [**linked_accounts_list**](docs/LinkedAccountsApi.md#linked_accounts_list) | **GET** /linked-accounts | 
*LocationsApi* | [**locations_list**](docs/LocationsApi.md#locations_list) | **GET** /locations | 
*LocationsApi* | [**locations_retrieve**](docs/LocationsApi.md#locations_retrieve) | **GET** /locations/{id} | 
*PassthroughApi* | [**passthrough_create**](docs/PassthroughApi.md#passthrough_create) | **POST** /passthrough | 
*PayGroupsApi* | [**pay_groups_list**](docs/PayGroupsApi.md#pay_groups_list) | **GET** /pay-groups | 
*PayGroupsApi* | [**pay_groups_retrieve**](docs/PayGroupsApi.md#pay_groups_retrieve) | **GET** /pay-groups/{id} | 
*PayrollRunsApi* | [**payroll_runs_list**](docs/PayrollRunsApi.md#payroll_runs_list) | **GET** /payroll-runs | 
*PayrollRunsApi* | [**payroll_runs_retrieve**](docs/PayrollRunsApi.md#payroll_runs_retrieve) | **GET** /payroll-runs/{id} | 
*RegenerateKeyApi* | [**regenerate_key_create**](docs/RegenerateKeyApi.md#regenerate_key_create) | **POST** /regenerate-key | 
*SelectiveSyncApi* | [**selective_sync_configurations_list**](docs/SelectiveSyncApi.md#selective_sync_configurations_list) | **GET** /selective-sync/configurations | 
*SelectiveSyncApi* | [**selective_sync_configurations_update**](docs/SelectiveSyncApi.md#selective_sync_configurations_update) | **PUT** /selective-sync/configurations | 
*SelectiveSyncApi* | [**selective_sync_meta_list**](docs/SelectiveSyncApi.md#selective_sync_meta_list) | **GET** /selective-sync/meta | 
*SyncStatusApi* | [**sync_status_list**](docs/SyncStatusApi.md#sync_status_list) | **GET** /sync-status | 
*TeamsApi* | [**teams_list**](docs/TeamsApi.md#teams_list) | **GET** /teams | 
*TeamsApi* | [**teams_retrieve**](docs/TeamsApi.md#teams_retrieve) | **GET** /teams/{id} | 
*TimeOffApi* | [**time_off_create**](docs/TimeOffApi.md#time_off_create) | **POST** /time-off | 
*TimeOffApi* | [**time_off_list**](docs/TimeOffApi.md#time_off_list) | **GET** /time-off | 
*TimeOffApi* | [**time_off_meta_post_retrieve**](docs/TimeOffApi.md#time_off_meta_post_retrieve) | **GET** /time-off/meta/post | 
*TimeOffApi* | [**time_off_retrieve**](docs/TimeOffApi.md#time_off_retrieve) | **GET** /time-off/{id} | 
*TimeOffBalancesApi* | [**time_off_balances_list**](docs/TimeOffBalancesApi.md#time_off_balances_list) | **GET** /time-off-balances | 
*TimeOffBalancesApi* | [**time_off_balances_retrieve**](docs/TimeOffBalancesApi.md#time_off_balances_retrieve) | **GET** /time-off-balances/{id} | 
*WebhookReceiversApi* | [**webhook_receivers_create**](docs/WebhookReceiversApi.md#webhook_receivers_create) | **POST** /webhook-receivers | 
*WebhookReceiversApi* | [**webhook_receivers_list**](docs/WebhookReceiversApi.md#webhook_receivers_list) | **GET** /webhook-receivers | 


## Documentation For Models

 - [AccountDetails](docs/AccountDetails.md)
 - [AccountDetailsAndActions](docs/AccountDetailsAndActions.md)
 - [AccountDetailsAndActionsIntegration](docs/AccountDetailsAndActionsIntegration.md)
 - [AccountDetailsAndActionsStatusEnum](docs/AccountDetailsAndActionsStatusEnum.md)
 - [AccountIntegration](docs/AccountIntegration.md)
 - [AccountToken](docs/AccountToken.md)
 - [AccountTypeEnum](docs/AccountTypeEnum.md)
 - [AvailableActions](docs/AvailableActions.md)
 - [BankInfo](docs/BankInfo.md)
 - [Benefit](docs/Benefit.md)
 - [CategoriesEnum](docs/CategoriesEnum.md)
 - [CategoryEnum](docs/CategoryEnum.md)
 - [Company](docs/Company.md)
 - [ConditionSchema](docs/ConditionSchema.md)
 - [ConditionTypeEnum](docs/ConditionTypeEnum.md)
 - [CountryEnum](docs/CountryEnum.md)
 - [DataPassthroughRequest](docs/DataPassthroughRequest.md)
 - [DebugModeLog](docs/DebugModeLog.md)
 - [DebugModelLogSummary](docs/DebugModelLogSummary.md)
 - [Deduction](docs/Deduction.md)
 - [Earning](docs/Earning.md)
 - [EarningTypeEnum](docs/EarningTypeEnum.md)
 - [Employee](docs/Employee.md)
 - [EmployeeEndpointRequest](docs/EmployeeEndpointRequest.md)
 - [EmployeePayrollRun](docs/EmployeePayrollRun.md)
 - [EmployeeRequest](docs/EmployeeRequest.md)
 - [EmployeeResponse](docs/EmployeeResponse.md)
 - [Employment](docs/Employment.md)
 - [EmploymentStatusEnum](docs/EmploymentStatusEnum.md)
 - [EmploymentTypeEnum](docs/EmploymentTypeEnum.md)
 - [EncodingEnum](docs/EncodingEnum.md)
 - [EndUserDetailsRequest](docs/EndUserDetailsRequest.md)
 - [ErrorValidationProblem](docs/ErrorValidationProblem.md)
 - [EthnicityEnum](docs/EthnicityEnum.md)
 - [FlsaStatusEnum](docs/FlsaStatusEnum.md)
 - [GenderEnum](docs/GenderEnum.md)
 - [GenerateRemoteKeyRequest](docs/GenerateRemoteKeyRequest.md)
 - [Group](docs/Group.md)
 - [GroupTypeEnum](docs/GroupTypeEnum.md)
 - [IgnoreCommonModel](docs/IgnoreCommonModel.md)
 - [IgnoreCommonModelRequest](docs/IgnoreCommonModelRequest.md)
 - [Issue](docs/Issue.md)
 - [IssueStatusEnum](docs/IssueStatusEnum.md)
 - [LinkToken](docs/LinkToken.md)
 - [LinkedAccountCondition](docs/LinkedAccountCondition.md)
 - [LinkedAccountConditionRequest](docs/LinkedAccountConditionRequest.md)
 - [LinkedAccountSelectiveSyncConfiguration](docs/LinkedAccountSelectiveSyncConfiguration.md)
 - [LinkedAccountSelectiveSyncConfigurationListRequest](docs/LinkedAccountSelectiveSyncConfigurationListRequest.md)
 - [LinkedAccountSelectiveSyncConfigurationRequest](docs/LinkedAccountSelectiveSyncConfigurationRequest.md)
 - [LinkedAccountStatus](docs/LinkedAccountStatus.md)
 - [Location](docs/Location.md)
 - [LocationTypeEnum](docs/LocationTypeEnum.md)
 - [MaritalStatusEnum](docs/MaritalStatusEnum.md)
 - [MetaResponse](docs/MetaResponse.md)
 - [MethodEnum](docs/MethodEnum.md)
 - [ModelOperation](docs/ModelOperation.md)
 - [MultipartFormFieldRequest](docs/MultipartFormFieldRequest.md)
 - [OperatorSchema](docs/OperatorSchema.md)
 - [PaginatedAccountDetailsAndActionsList](docs/PaginatedAccountDetailsAndActionsList.md)
 - [PaginatedBankInfoList](docs/PaginatedBankInfoList.md)
 - [PaginatedBenefitList](docs/PaginatedBenefitList.md)
 - [PaginatedCompanyList](docs/PaginatedCompanyList.md)
 - [PaginatedConditionSchemaList](docs/PaginatedConditionSchemaList.md)
 - [PaginatedDeductionList](docs/PaginatedDeductionList.md)
 - [PaginatedEmployeeList](docs/PaginatedEmployeeList.md)
 - [PaginatedEmployeePayrollRunList](docs/PaginatedEmployeePayrollRunList.md)
 - [PaginatedEmploymentList](docs/PaginatedEmploymentList.md)
 - [PaginatedGroupList](docs/PaginatedGroupList.md)
 - [PaginatedIssueList](docs/PaginatedIssueList.md)
 - [PaginatedLocationList](docs/PaginatedLocationList.md)
 - [PaginatedPayGroupList](docs/PaginatedPayGroupList.md)
 - [PaginatedPayrollRunList](docs/PaginatedPayrollRunList.md)
 - [PaginatedSyncStatusList](docs/PaginatedSyncStatusList.md)
 - [PaginatedTeamList](docs/PaginatedTeamList.md)
 - [PaginatedTimeOffBalanceList](docs/PaginatedTimeOffBalanceList.md)
 - [PaginatedTimeOffList](docs/PaginatedTimeOffList.md)
 - [PayCurrencyEnum](docs/PayCurrencyEnum.md)
 - [PayFrequencyEnum](docs/PayFrequencyEnum.md)
 - [PayGroup](docs/PayGroup.md)
 - [PayPeriodEnum](docs/PayPeriodEnum.md)
 - [PayrollRun](docs/PayrollRun.md)
 - [PolicyTypeEnum](docs/PolicyTypeEnum.md)
 - [ReasonEnum](docs/ReasonEnum.md)
 - [RemoteData](docs/RemoteData.md)
 - [RemoteKey](docs/RemoteKey.md)
 - [RemoteKeyForRegenerationRequest](docs/RemoteKeyForRegenerationRequest.md)
 - [RemoteResponse](docs/RemoteResponse.md)
 - [RequestFormatEnum](docs/RequestFormatEnum.md)
 - [RequestTypeEnum](docs/RequestTypeEnum.md)
 - [ResponseTypeEnum](docs/ResponseTypeEnum.md)
 - [RunStateEnum](docs/RunStateEnum.md)
 - [RunTypeEnum](docs/RunTypeEnum.md)
 - [SelectiveSyncConfigurationsUsageEnum](docs/SelectiveSyncConfigurationsUsageEnum.md)
 - [SyncStatus](docs/SyncStatus.md)
 - [SyncStatusStatusEnum](docs/SyncStatusStatusEnum.md)
 - [Tax](docs/Tax.md)
 - [Team](docs/Team.md)
 - [TimeOff](docs/TimeOff.md)
 - [TimeOffBalance](docs/TimeOffBalance.md)
 - [TimeOffEndpointRequest](docs/TimeOffEndpointRequest.md)
 - [TimeOffRequest](docs/TimeOffRequest.md)
 - [TimeOffResponse](docs/TimeOffResponse.md)
 - [TimeOffStatusEnum](docs/TimeOffStatusEnum.md)
 - [UnitsEnum](docs/UnitsEnum.md)
 - [ValidationProblemSource](docs/ValidationProblemSource.md)
 - [WarningValidationProblem](docs/WarningValidationProblem.md)
 - [WebhookReceiver](docs/WebhookReceiver.md)
 - [WebhookReceiverRequest](docs/WebhookReceiverRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

hello@merge.dev


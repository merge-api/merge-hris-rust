/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`issues_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuesListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`issues_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuesRetrieveError {
    UnknownValue(serde_json::Value),
}


/// Gets issues.
pub async fn issues_list(configuration: &configuration::Configuration, account_token: Option<&str>, cursor: Option<&str>, end_date: Option<&str>, end_user_organization_name: Option<&str>, first_incident_time_after: Option<String>, first_incident_time_before: Option<String>, include_muted: Option<&str>, integration_name: Option<&str>, last_incident_time_after: Option<String>, last_incident_time_before: Option<String>, page_size: Option<i32>, start_date: Option<&str>, status: Option<&str>) -> Result<crate::models::PaginatedIssueList, Error<IssuesListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/issues", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = account_token {
        local_var_req_builder = local_var_req_builder.query(&[("account_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date {
        local_var_req_builder = local_var_req_builder.query(&[("end_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_user_organization_name {
        local_var_req_builder = local_var_req_builder.query(&[("end_user_organization_name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = first_incident_time_after {
        local_var_req_builder = local_var_req_builder.query(&[("first_incident_time_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = first_incident_time_before {
        local_var_req_builder = local_var_req_builder.query(&[("first_incident_time_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_muted {
        local_var_req_builder = local_var_req_builder.query(&[("include_muted", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = integration_name {
        local_var_req_builder = local_var_req_builder.query(&[("integration_name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = last_incident_time_after {
        local_var_req_builder = local_var_req_builder.query(&[("last_incident_time_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = last_incident_time_before {
        local_var_req_builder = local_var_req_builder.query(&[("last_incident_time_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_date {
        local_var_req_builder = local_var_req_builder.query(&[("start_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IssuesListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific issue.
pub async fn issues_retrieve(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::Issue, Error<IssuesRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/issues/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IssuesRetrieveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


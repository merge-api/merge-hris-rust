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


/// struct for typed errors of method [`locations_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocationsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`locations_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocationsRetrieveError {
    UnknownValue(serde_json::Value),
}


/// Returns a list of `Location` objects.
pub async fn locations_list(configuration: &configuration::Configuration, x_account_token: &str, created_after: Option<String>, created_before: Option<String>, cursor: Option<&str>, include_deleted_data: Option<bool>, include_remote_data: Option<bool>, modified_after: Option<String>, modified_before: Option<String>, page_size: Option<i32>, remote_fields: Option<&str>, remote_id: Option<&str>) -> Result<crate::models::PaginatedLocationList, Error<LocationsListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/locations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = created_after {
        local_var_req_builder = local_var_req_builder.query(&[("created_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_before {
        local_var_req_builder = local_var_req_builder.query(&[("created_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_deleted_data {
        local_var_req_builder = local_var_req_builder.query(&[("include_deleted_data", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_remote_data {
        local_var_req_builder = local_var_req_builder.query(&[("include_remote_data", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = modified_after {
        local_var_req_builder = local_var_req_builder.query(&[("modified_after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = modified_before {
        local_var_req_builder = local_var_req_builder.query(&[("modified_before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = remote_fields {
        local_var_req_builder = local_var_req_builder.query(&[("remote_fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = remote_id {
        local_var_req_builder = local_var_req_builder.query(&[("remote_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Account-Token", x_account_token.to_string());
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
        let local_var_entity: Option<LocationsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a `Location` object with the given `id`.
pub async fn locations_retrieve(configuration: &configuration::Configuration, x_account_token: &str, id: &str, include_remote_data: Option<bool>, remote_fields: Option<&str>) -> Result<crate::models::Location, Error<LocationsRetrieveError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/locations/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_remote_data {
        local_var_req_builder = local_var_req_builder.query(&[("include_remote_data", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = remote_fields {
        local_var_req_builder = local_var_req_builder.query(&[("remote_fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Account-Token", x_account_token.to_string());
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
        let local_var_entity: Option<LocationsRetrieveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountToken {
    #[serde(rename = "account_token")]
    pub account_token: String,
    #[serde(rename = "integration")]
    pub integration: Box<crate::models::AccountIntegration>,
}

impl AccountToken {
    pub fn new(account_token: String, integration: crate::models::AccountIntegration) -> AccountToken {
        AccountToken {
            account_token,
            integration: Box::new(integration),
        }
    }
}


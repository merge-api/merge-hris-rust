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
pub struct PaginatedTeamList {
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::models::Team>>,
}

impl PaginatedTeamList {
    pub fn new() -> PaginatedTeamList {
        PaginatedTeamList {
            next: None,
            previous: None,
            results: None,
        }
    }
}



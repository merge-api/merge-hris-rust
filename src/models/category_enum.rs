/*
 * Merge HRIS API
 *
 * The unified API for building rich integrations with multiple HR Information System platforms.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: hello@merge.dev
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CategoryEnum {
    #[serde(rename = "hris")]
    Hris,
    #[serde(rename = "ats")]
    Ats,
    #[serde(rename = "accounting")]
    Accounting,
    #[serde(rename = "ticketing")]
    Ticketing,
    #[serde(rename = "crm")]
    Crm,

}

impl ToString for CategoryEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Hris => String::from("hris"),
            Self::Ats => String::from("ats"),
            Self::Accounting => String::from("accounting"),
            Self::Ticketing => String::from("ticketing"),
            Self::Crm => String::from("crm"),
        }
    }
}

impl Default for CategoryEnum {
    fn default() -> CategoryEnum {
        Self::Hris
    }
}





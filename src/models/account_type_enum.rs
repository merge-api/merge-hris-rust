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
pub enum AccountTypeEnum {
    #[serde(rename = "SAVINGS")]
    SAVINGS,
    #[serde(rename = "CHECKING")]
    CHECKING,

}

impl ToString for AccountTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::SAVINGS => String::from("SAVINGS"),
            Self::CHECKING => String::from("CHECKING"),
        }
    }
}

impl Default for AccountTypeEnum {
    fn default() -> AccountTypeEnum {
        Self::SAVINGS
    }
}





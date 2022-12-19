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
pub enum AccountDetailsAndActionsStatusEnum {
    #[serde(rename = "COMPLETE")]
    COMPLETE,
    #[serde(rename = "INCOMPLETE")]
    INCOMPLETE,
    #[serde(rename = "RELINK_NEEDED")]
    RELINKNEEDED,

}

impl ToString for AccountDetailsAndActionsStatusEnum {
    fn to_string(&self) -> String {
        match self {
            Self::COMPLETE => String::from("COMPLETE"),
            Self::INCOMPLETE => String::from("INCOMPLETE"),
            Self::RELINKNEEDED => String::from("RELINK_NEEDED"),
        }
    }
}

impl Default for AccountDetailsAndActionsStatusEnum {
    fn default() -> AccountDetailsAndActionsStatusEnum {
        Self::COMPLETE
    }
}





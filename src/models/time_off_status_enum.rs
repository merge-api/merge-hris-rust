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
pub enum TimeOffStatusEnum {
    #[serde(rename = "REQUESTED")]
    REQUESTED,
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "DECLINED")]
    DECLINED,
    #[serde(rename = "CANCELLED")]
    CANCELLED,
    #[serde(rename = "DELETED")]
    DELETED,

}

impl ToString for TimeOffStatusEnum {
    fn to_string(&self) -> String {
        match self {
            Self::REQUESTED => String::from("REQUESTED"),
            Self::APPROVED => String::from("APPROVED"),
            Self::DECLINED => String::from("DECLINED"),
            Self::CANCELLED => String::from("CANCELLED"),
            Self::DELETED => String::from("DELETED"),
        }
    }
}

impl Default for TimeOffStatusEnum {
    fn default() -> TimeOffStatusEnum {
        Self::REQUESTED
    }
}





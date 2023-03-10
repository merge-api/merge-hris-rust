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
pub enum RunStateEnum {
    #[serde(rename = "PAID")]
    PAID,
    #[serde(rename = "DRAFT")]
    DRAFT,
    #[serde(rename = "APPROVED")]
    APPROVED,
    #[serde(rename = "FAILED")]
    FAILED,
    #[serde(rename = "CLOSED")]
    CLOSED,

}

impl ToString for RunStateEnum {
    fn to_string(&self) -> String {
        match self {
            Self::PAID => String::from("PAID"),
            Self::DRAFT => String::from("DRAFT"),
            Self::APPROVED => String::from("APPROVED"),
            Self::FAILED => String::from("FAILED"),
            Self::CLOSED => String::from("CLOSED"),
        }
    }
}

impl Default for RunStateEnum {
    fn default() -> RunStateEnum {
        Self::PAID
    }
}





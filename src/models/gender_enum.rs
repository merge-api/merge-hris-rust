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
pub enum GenderEnum {
    #[serde(rename = "MALE")]
    MALE,
    #[serde(rename = "FEMALE")]
    FEMALE,
    #[serde(rename = "NON-BINARY")]
    NONBINARY,
    #[serde(rename = "OTHER")]
    OTHER,
    #[serde(rename = "PREFER_NOT_TO_DISCLOSE")]
    PREFERNOTTODISCLOSE,

}

impl ToString for GenderEnum {
    fn to_string(&self) -> String {
        match self {
            Self::MALE => String::from("MALE"),
            Self::FEMALE => String::from("FEMALE"),
            Self::NONBINARY => String::from("NON-BINARY"),
            Self::OTHER => String::from("OTHER"),
            Self::PREFERNOTTODISCLOSE => String::from("PREFER_NOT_TO_DISCLOSE"),
        }
    }
}

impl Default for GenderEnum {
    fn default() -> GenderEnum {
        Self::MALE
    }
}





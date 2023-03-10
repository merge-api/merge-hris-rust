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
pub enum ConditionTypeEnum {
    #[serde(rename = "BOOLEAN")]
    BOOLEAN,
    #[serde(rename = "DATE")]
    DATE,
    #[serde(rename = "DATE_TIME")]
    DATETIME,
    #[serde(rename = "INTEGER")]
    INTEGER,
    #[serde(rename = "FLOAT")]
    FLOAT,
    #[serde(rename = "STRING")]
    STRING,
    #[serde(rename = "LIST_OF_STRINGS")]
    LISTOFSTRINGS,

}

impl ToString for ConditionTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::BOOLEAN => String::from("BOOLEAN"),
            Self::DATE => String::from("DATE"),
            Self::DATETIME => String::from("DATE_TIME"),
            Self::INTEGER => String::from("INTEGER"),
            Self::FLOAT => String::from("FLOAT"),
            Self::STRING => String::from("STRING"),
            Self::LISTOFSTRINGS => String::from("LIST_OF_STRINGS"),
        }
    }
}

impl Default for ConditionTypeEnum {
    fn default() -> ConditionTypeEnum {
        Self::BOOLEAN
    }
}





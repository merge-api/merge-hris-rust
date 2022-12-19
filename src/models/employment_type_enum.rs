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
pub enum EmploymentTypeEnum {
    #[serde(rename = "FULL_TIME")]
    FULLTIME,
    #[serde(rename = "PART_TIME")]
    PARTTIME,
    #[serde(rename = "INTERN")]
    INTERN,
    #[serde(rename = "CONTRACTOR")]
    CONTRACTOR,
    #[serde(rename = "FREELANCE")]
    FREELANCE,

}

impl ToString for EmploymentTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::FULLTIME => String::from("FULL_TIME"),
            Self::PARTTIME => String::from("PART_TIME"),
            Self::INTERN => String::from("INTERN"),
            Self::CONTRACTOR => String::from("CONTRACTOR"),
            Self::FREELANCE => String::from("FREELANCE"),
        }
    }
}

impl Default for EmploymentTypeEnum {
    fn default() -> EmploymentTypeEnum {
        Self::FULLTIME
    }
}





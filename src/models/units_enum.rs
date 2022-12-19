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
pub enum UnitsEnum {
    #[serde(rename = "HOURS")]
    HOURS,
    #[serde(rename = "DAYS")]
    DAYS,

}

impl ToString for UnitsEnum {
    fn to_string(&self) -> String {
        match self {
            Self::HOURS => String::from("HOURS"),
            Self::DAYS => String::from("DAYS"),
        }
    }
}

impl Default for UnitsEnum {
    fn default() -> UnitsEnum {
        Self::HOURS
    }
}





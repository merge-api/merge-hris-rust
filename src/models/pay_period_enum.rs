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
pub enum PayPeriodEnum {
    #[serde(rename = "HOUR")]
    HOUR,
    #[serde(rename = "DAY")]
    DAY,
    #[serde(rename = "WEEK")]
    WEEK,
    #[serde(rename = "EVERY_TWO_WEEKS")]
    EVERYTWOWEEKS,
    #[serde(rename = "SEMIMONTHLY")]
    SEMIMONTHLY,
    #[serde(rename = "MONTH")]
    MONTH,
    #[serde(rename = "QUARTER")]
    QUARTER,
    #[serde(rename = "EVERY_SIX_MONTHS")]
    EVERYSIXMONTHS,
    #[serde(rename = "YEAR")]
    YEAR,

}

impl ToString for PayPeriodEnum {
    fn to_string(&self) -> String {
        match self {
            Self::HOUR => String::from("HOUR"),
            Self::DAY => String::from("DAY"),
            Self::WEEK => String::from("WEEK"),
            Self::EVERYTWOWEEKS => String::from("EVERY_TWO_WEEKS"),
            Self::SEMIMONTHLY => String::from("SEMIMONTHLY"),
            Self::MONTH => String::from("MONTH"),
            Self::QUARTER => String::from("QUARTER"),
            Self::EVERYSIXMONTHS => String::from("EVERY_SIX_MONTHS"),
            Self::YEAR => String::from("YEAR"),
        }
    }
}

impl Default for PayPeriodEnum {
    fn default() -> PayPeriodEnum {
        Self::HOUR
    }
}





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
pub enum RequestFormatEnum {
    #[serde(rename = "JSON")]
    JSON,
    #[serde(rename = "XML")]
    XML,
    #[serde(rename = "MULTIPART")]
    MULTIPART,

}

impl ToString for RequestFormatEnum {
    fn to_string(&self) -> String {
        match self {
            Self::JSON => String::from("JSON"),
            Self::XML => String::from("XML"),
            Self::MULTIPART => String::from("MULTIPART"),
        }
    }
}

impl Default for RequestFormatEnum {
    fn default() -> RequestFormatEnum {
        Self::JSON
    }
}





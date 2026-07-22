use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AgsScope {
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem")]
    LineItem,
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-ags/scope/result.readonly")]
    ResultReadOnly,
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-ags/scope/score")]
    Score,
}

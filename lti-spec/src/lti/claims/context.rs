use serde::{Deserialize, Serialize};

/// A context is a grouping of resources, such as a course.
/// **Claim:** `https://purl.imsglobal.org/spec/lti/claim/context`
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Context {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_type: Option<ContextType>,

    /// This is often the course code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,

    /// an identifier unique to both the deployment_id and issuer (iss) claims.
    pub id: Box<str>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ContextType {
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/course#CourseTemplate")]
    CourseTemplate,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/course#CourseOffering")]
    CourseOffering,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/course#CourseSection")]
    CourseSection,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/course#Group")]
    Group,
    #[serde(untagged)]
    Other(Box<str>),
}

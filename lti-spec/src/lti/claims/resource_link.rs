use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceLink {
    /// Opaque identifier for a placement of an LTI resource link within a context, unique per
    /// deployment_id.
    pub id: Box<str>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
}

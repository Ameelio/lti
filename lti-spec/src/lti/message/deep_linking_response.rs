use serde::{Deserialize, Serialize};

use crate::lti::claims::prelude::*;
use crate::oidc::user_claims::UserClaims;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeepLinkingResponse {
    #[serde(flatten)]
    pub auth: ResponseAuth,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti-dl/claim/content_items")]
    pub content_items: Box<[ContentItem]>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/deployment_id")]
    pub deployment_id: Box<str>,

    #[serde(default, flatten)]
    pub options: DeepLinkingResponseOptions,

    #[serde(flatten)]
    pub user: UserClaims,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeepLinkingResponseOptions {
    /// An opaque value that must match the data claim provided in the request.
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-dl/claim/data")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<str>>,

    /// error message the platorm may log.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-dl/claim/errorlog")]
    pub error_log: Option<Box<str>>,

    /// error message the platform may show the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-dl/claim/errormsg")]
    pub error_msg: Option<Box<str>>,

    /// message the platorm may log.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-dl/claim/log")]
    pub log: Option<Box<str>>,

    #[serde(default)]
    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/message_type")]
    pub message_type: MessageType,

    /// Plain text string the platform may show the end user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-dl/claim/msg")]
    pub msg: Option<Box<str>>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/version")]
    pub version: Version,
}

#[derive(Clone, Debug, Deserialize, Default, Eq, PartialEq, Serialize)]
pub enum MessageType {
    #[default]
    LtiDeepLinkingResponse,
}

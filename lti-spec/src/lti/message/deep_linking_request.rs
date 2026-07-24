use serde::{Deserialize, Serialize};

use crate::lti::claims::prelude::*;
use crate::oidc::auth_claims::AuthClaims;
use crate::oidc::user_claims::UserClaims;
use crate::prelude::AgsEndpoint;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeepLinkingRequest<C = CustomParams> {
    #[serde(flatten)]
    pub auth: AuthClaims,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti-dl/claim/deep_linking_settings")]
    pub deep_linking_settings: DeepLinkingSettings,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/deployment_id")]
    pub deployment_id: Box<str>,

    #[serde(default, flatten)]
    pub options: DeepLinkingRequestOptions<C>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/roles")]
    pub roles: Box<[Role]>,

    #[serde(flatten)]
    pub user: UserClaims,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeepLinkingRequestOptions<C = CustomParams> {
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-ags/claim/endpoint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ags_endpoint: Option<AgsEndpoint>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/context")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    #[serde(rename = "http://imsglobal.org/custom ")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<C>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/launch_presentation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_presentation: Option<LaunchPresentation>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/role_scope_mentor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentees: Option<Box<[Box<str>]>>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/message_type")]
    #[serde(default)]
    pub message_type: MessageType,

    #[serde(rename = "http://purl.imsglobal.org/lti/claim/tool_platform")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformInstance>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/version")]
    pub version: Version,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum MessageType {
    #[default]
    LtiDeepLinkingRequest,
}

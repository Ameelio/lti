use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::lti::claims::prelude::*;
use crate::oidc::auth_claims::AuthClaims;
use crate::oidc::user_claims::UserClaims;
use crate::prelude::AgsEndpoint;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceLinkRequest {
    #[serde(flatten)]
    pub auth: AuthClaims,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/deployment_id")]
    pub deployment_id: Box<str>,

    #[serde(flatten)]
    pub options: ResourceLinkRequestOptions,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/resource_link")]
    pub resource_link: ResourceLink,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/roles")]
    pub roles: Box<[Role]>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/target_link_uri")]
    pub target_link_uri: Url,

    #[serde(flatten)]
    pub user: UserClaims,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/version")]
    pub version: Version,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceLinkRequestOptions {
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-ags/claim/endpoint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ags_endpoint: Option<AgsEndpoint>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/context")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,

    #[serde(rename = "http://imsglobal.org/custom ")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<Box<str>, Box<str>>>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/launch_presentation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_presentation: Option<LaunchPresentation>,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti/claim/role_scope_mentor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentees: Option<Box<[Box<str>]>>,

    #[serde(rename = "http://purl.imsglobal.org/lti/claim/tool_platform")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformInstance>,
}

//! handles the LTI HTTP API
//!

use serde::{Deserialize, Serialize};
use url::Url;

use crate::domain::Domain;
use crate::lti::MessageConfig;
use crate::lti::claims::canvas_privacy_level::CanvasPrivacyLevel;
use crate::lti::claims::custom_params::CustomParams;
use crate::oidc::user_claims::SupportedUserClaims;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ToolConfiguration {
    pub domain: Domain,
    #[serde(flatten)]
    pub options: ToolConfigurationOptions,
    pub messages: Box<[MessageConfig]>,
    pub target_link_uri: Url,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ToolConfigurationOptions {
    pub claims: Box<[SupportedUserClaims]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<CustomParams>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_domains: Option<Box<[Domain]>>,
    #[serde(rename = "https://canvas.instructure.com/lti/privacy_level")]
    #[serde(default, skip_serializing_if = "CanvasPrivacyLevel::is_anonymous")]
    pub privacy_level: CanvasPrivacyLevel,
}

impl Default for ToolConfigurationOptions {
    fn default() -> Self {
        let claims: Box<[SupportedUserClaims]> = [SupportedUserClaims::Subject].into();

        Self {
            claims,
            custom_parameters: None,
            description: None,
            secondary_domains: None,
            privacy_level: CanvasPrivacyLevel::Anonymous,
        }
    }
}

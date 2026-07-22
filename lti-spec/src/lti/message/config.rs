//! Message configuration
//! See [Lti Message Spec](https://www.imsglobal.org/spec/lti-dr/v1p0#lti-message)
mod deep_linking_config;
mod resource_link_config;

use serde::{Deserialize, Serialize};
use url::Url;

pub use deep_linking_config::DeepLinkingConfig;
pub use resource_link_config::ResourceLinkConfig;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum MessageConfig {
    LtiResourceLinkRequest(ResourceLinkConfig),
    LtiDeepLinkingRequest(DeepLinkingConfig),
}

impl MessageConfig {
    pub fn target_link_uri(&self) -> Option<&Url> {
        match self {
            Self::LtiResourceLinkRequest(m) => m.target_link_uri.as_ref(),
            Self::LtiDeepLinkingRequest(m) => m.target_link_uri.as_ref(),
        }
    }
}

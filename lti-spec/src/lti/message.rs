//! Message configuration
//! See [Lti Message Spec](https://www.imsglobal.org/spec/lti-dr/v1p0#lti-message)

mod config;
mod deep_linking_request;
mod deep_linking_response;
mod resource_link_request;
mod submission_review_request;

pub use config::{DeepLinkingConfig, MessageConfig, ResourceLinkConfig};
pub use deep_linking_request::{DeepLinkingRequest, DeepLinkingRequestOptions};
pub use deep_linking_response::{DeepLinkingResponse, DeepLinkingResponseOptions};
pub use resource_link_request::ResourceLinkRequest;
pub use submission_review_request::{SubmissionReviewRequest, SubmissionReviewRequestOptions};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "https://purl.imsglobal.org/spec/lti/claim/message_type")]
pub enum Message {
    LtiDeepLinkingResponse(DeepLinkingResponse),
    LtiDeepLinkingRequest(DeepLinkingRequest),
    LtiResourceLinkRequest(ResourceLinkRequest),
    LtiSubmissionReviewRequest(SubmissionReviewRequest),
}

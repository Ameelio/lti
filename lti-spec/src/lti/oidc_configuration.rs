use serde::{Deserialize, Serialize};
use url::Url;

use crate::{domain::Domain, oidc::*};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OidcConfiguration {
    pub issuer: Url,
    pub authorization_endpoint: Url,
    pub authorization_server: Option<Domain>,
    pub claims_supported: Box<[OpenIdClaim]>,
    pub id_token_signing_alg_values_supported: Box<[SigningAlg]>,
    pub jwks_uri: Url,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-platform-configuration")]
    pub lti_platform_configuration: LtiPlatformConfiguration,
    pub registration_endpoint: Url,
    pub response_types_supported: Box<[ResponseType]>,
    pub scopes_supported: Box<[SupportedScope]>,
    pub subject_types_supported: Box<[SubjectType]>,
    pub token_endpoint: Url,
    pub token_endpoint_auth_methods_supported: Box<[TokenAuthMethod]>,
    pub token_endpoint_auth_signing_alg_values_supported: Box<[SigningAlg]>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedScope {
    OpenId,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-reg/scope/registration.readonly")]
    RegistrationReadonly,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-reg/scope/registration")]
    Registration,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem")]
    LineItem,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem.readonly")]
    LineItemReadonly,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-ags/scope/result.readonly")]
    ResultReadonly,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-ags/scope/score")]
    Score,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti-nrps/scope/contextmembership.readonly")]
    ContextMembershipReadonly,
    #[serde(alias = "https://purl.imsglobal.org/spec/lti/scope/noticehandlers")]
    NoticeHandlers,
    #[serde(alias = "https://canvas.instructure.com/lti/public_jwk/scope/update")]
    CanvasPublicJwkUpdate,
    #[serde(alias = "https://canvas.instructure.com/lti/account_lookup/scope/show")]
    CanvasAccountLookupShow,
    #[serde(alias = "https://canvas.instructure.com/lti-ags/progress/scope/show")]
    CanvasProgressShow,
    #[serde(alias = "https://canvas.instructure.com/lti/page_content/show")]
    CanvasPageContentShow,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubjectType {
    Public,
    Pairwise,
    #[serde(other)]
    Unsupported,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LtiPlatformConfiguration {
    pub product_family_code: Box<str>,
    pub version: Box<str>,
    pub messages_supported: Box<[Message]>,
    pub variables: Box<[Box<str>]>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Message {
    LtiResourceLinkRequest {
        placements: Box<[Box<str>]>,
    },
    LtiDeepLinkingRequest {
        placements: Box<[Box<str>]>,
    },
    LtiDeepLinkingResponse {
        placements: Box<[Box<str>]>,
    },
    #[serde(other)]
    Unsupported,
}

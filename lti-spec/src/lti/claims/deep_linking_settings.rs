use serde::{Deserialize, Serialize};
use url::Url;

use crate::lti::claims::content_item::ContentItemType;
use crate::util::Toggle;

/// This is a claim provided in a json web token with
/// the return_url and other settings related to an LTI
/// deep linking request.
/// See: [Deep linking settings](https://www.imsglobal.org/spec/lti-dl/v2p0#deep-linking-settings)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeepLinkingSettings {
    /// Array of supported types accepted.
    #[serde(default)]
    pub accept_types: Box<[ContentItemType]>,
    /// Array of supported targets in the browser (i.e. iframe or window)
    #[serde(default)]
    pub accept_presentation_document_targets: Box<[HtmlTarget]>,
    /// Comma seperated list of mime types supported, specifically for the File ContentItemType.
    #[serde(default)]
    pub accept_media_types: Option<Box<str>>,
    /// true if the platform allows sending multiple content items in
    /// a single response.
    #[serde(default)]
    pub accept_multiple: Toggle,
    /// true if the platform supports line items included in a Lti
    /// Resource Link.
    #[serde(default)]
    pub accept_lineitem: Toggle,
    /// true if content items returned by the tool are automatically persisted.
    #[serde(default)]
    pub auto_create: bool,
    /// opaque value that should just get sent with the response if present.
    #[serde(default)]
    pub data: Option<Box<str>>,
    /// the link to send a POST response
    pub deep_link_return_url: Url,
    /// default text content for the content item returned by the tool.
    #[serde(default)]
    pub text: Option<Box<str>>,
    /// default title of the content item returned by the tool.
    #[serde(default)]
    pub title: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HtmlTarget {
    Embed,
    Iframe,
    Window,
}

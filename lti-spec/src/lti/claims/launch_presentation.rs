use serde::{Deserialize, Serialize};
use url::Url;

/// How the message sender expects to host the receiver's content.
/// **Claim:** `https://purl.imsglobal.org/spec/lti/claim/launch_presentation`
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LaunchPresentation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    document_target: Option<DocumentTarget>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    height: Option<u16>,

    /// Url where the receiver can redirect the user afterwards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    return_url: Option<Url>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    width: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum DocumentTarget {
    Frame,
    Iframe,
    Window,
    Unsupported(Box<str>),
}

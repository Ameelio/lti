use serde::{Deserialize, Serialize};
use url::Url;

use super::icon::Icon;
use super::iframe::Iframe;
use super::window::Window;
use crate::util::SanitizedHtml;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Link {
    #[serde(default, flatten)]
    pub options: LinkOptions,
    pub url: Url,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LinkOptions {
    /// html fragment to embed the resource into, must be sanitized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed: Option<SanitizedHtml>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iframe: Option<Iframe>,
    /// description of the content item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<Window>,
}

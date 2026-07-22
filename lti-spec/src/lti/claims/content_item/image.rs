use serde::{Deserialize, Serialize};
use url::Url;

use super::icon::Icon;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Image {
    #[serde(default, flatten)]
    pub options: ImageOptions,
    pub url: Url,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImageOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// description of the content item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u16>,
}

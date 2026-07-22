use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use super::icon::Icon;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct File {
    #[serde(flatten, default)]
    pub options: FileOptions,
    pub url: Url,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FileOptions {
    /// Garunteed the url will be available until this date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// description of the content item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
}

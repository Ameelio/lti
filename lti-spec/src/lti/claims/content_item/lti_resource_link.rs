use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::lti::LineItem;
use crate::lti::claims::prelude::CustomParams;
use crate::util::SanitizedHtml;

use super::icon::Icon;
use super::iframe::Iframe;
use super::window::Window;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LtiResourceLink {
    #[serde(default, flatten)]
    pub options: LtiResourceLinkOptions,
    pub url: Url,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LtiResourceLinkOptions {
    /// Window of time this activity is available to learners.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<TimeWindow>,

    /// Map of custom parameters, the parameter keys must be included
    /// in the request payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomParams>,

    /// html fragment to embed the resource into, must be sanitized.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed: Option<SanitizedHtml>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iframe: Option<Iframe>,

    /// This allows the activity to receive scores.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lineItem")]
    pub line_item: Option<LineItem>,

    /// Window of time submissions for this activity is available for learners.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission: Option<TimeWindow>,

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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TimeWindow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endDateTime")]
    pub end: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startDateTime")]
    pub start: Option<DateTime<Utc>>,
}

impl LtiResourceLinkOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_custom_param(
        mut self,
        name: impl Into<Box<str>>,
        mapping: impl Into<Box<str>>,
    ) -> Self {
        let mut params: CustomParams = self.custom.unwrap_or_default();

        params.insert(name.into(), mapping.into());

        self.custom = Some(params);

        self
    }
}

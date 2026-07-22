//! Content Items for Lti Deep Linking
//! See: [Content Item Types](https://www.imsglobal.org/spec/lti-dl/v2p0#content-item-types)
//!
pub mod embed;
pub mod icon;

mod file;
mod html_fragment;
mod iframe;
mod image;
mod link;
mod lti_resource_link;
mod window;

pub use file::{File, FileOptions};
pub use html_fragment::HtmlFragment;
pub use image::{Image, ImageOptions};
pub use link::{Link, LinkOptions};
pub use lti_resource_link::{LtiResourceLink, LtiResourceLinkOptions, TimeWindow};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ContentItem {
    #[serde(rename = "html")]
    HtmlFragment(HtmlFragment),
    #[serde(rename = "link")]
    Link(Link),
    #[serde(rename = "file")]
    File(File),
    #[serde(rename = "image")]
    Image(Image),
    #[serde(rename = "ltiResourceLink")]
    LtiResourceLink(LtiResourceLink),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ContentItemType {
    #[serde(rename = "html")]
    HtmlFragment,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "ltiResourceLink")]
    LtiResourceLink,
}

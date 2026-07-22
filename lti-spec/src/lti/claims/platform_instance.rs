use serde::{Deserialize, Serialize};
use url::Url;

/// Metadata for the platform instance.
/// A platform in this case refers to the Learn Managment System
/// such as Instructure Canvas.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformInstance {
    /// Administrative contact email for this instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Box<str>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,

    /// A string unique to the issuer (iss) claim of this instance.
    pub guid: Box<str>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,

    /// Vendor product family code for the type of platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_family_code: Option<Box<str>>,

    /// Home HTTPS endpoint of the platform instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,

    /// Vendor product version of the platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<str>>,
}

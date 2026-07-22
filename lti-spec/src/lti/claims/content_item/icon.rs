use serde::{Deserialize, Serialize};
use url::Url;

/// An icon to display for a content item.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Icon {
    /// height of the image in pixels
    pub height: u64,
    /// Url to the image file
    pub url: Url,
    /// width of the image in pixels
    pub width: u64,
}

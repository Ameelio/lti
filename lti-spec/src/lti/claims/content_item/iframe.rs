use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Iframe {
    pub height: u64,
    pub width: u64,
    pub src: Url,
}

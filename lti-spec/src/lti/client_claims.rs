use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientClaims {
    pub aud: Aud,

    #[serde(rename = "iss")]
    pub issuer: Url,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Aud {
    Single(Box<str>),
    List(Box<[Box<str>]>),
}

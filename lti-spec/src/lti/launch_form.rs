use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct LaunchForm {
    pub client_id: String,
    #[serde(alias = "iss")]
    pub issuer: String,
    pub lti_message_hint: Option<String>,
    pub login_hint: Option<String>,
    pub target_link_uri: Option<Url>,
}

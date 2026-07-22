use std::sync::Arc;
use url::Url;

#[derive(Clone, Debug)]
pub struct LmsClient {
    pub client_id: Arc<str>,
    pub jwks_url: Arc<Url>,
    pub key_id: Arc<str>,
}

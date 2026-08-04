use crate::lms_client::LmsClient;
use axum::http::request::Parts;
use json_web_key::jwk::JsonWebKeySet;
use json_web_tolkien::jws::Jws;

pub trait ClaimsState: std::fmt::Debug {
    fn fetch_client(
        &self,
        client_ids: Vec<&str>,
        issuer: &str,
        key_id: &str,
    ) -> Result<LmsClient, Box<dyn std::error::Error>>;

    fn protect_against_forgery(
        &self,
        actual: &str,
        parts: &mut Parts,
    ) -> impl Future<Output = CsrfResult> + Send;

    fn try_lock(&self, nonce: &str) -> impl Future<Output = LockResult> + Send;

    fn validate_signature(
        &self,
        key_id: &str,
        keys: JsonWebKeySet,
        signature: Jws,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub enum CsrfResult {
    Ok,
    Invalid,
    Missing,
}

pub enum LockResult {
    Ok,
    DuplicateRequest,
    Err(Box<dyn std::error::Error + Send + Sync + 'static>),
}

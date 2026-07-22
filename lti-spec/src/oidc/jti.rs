use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a token.
/// ## Example
///   use lti_spec::oidc::Jti;
///
///   let jti = Jti::generate();
#[derive(Debug, Deserialize, Serialize)]
pub struct Jti(Box<str>);

impl Jti {
    pub fn generate() -> Self {
        let jti = Uuid::now_v7();
        let jti = jti.to_string();
        let jti = jti.into_boxed_str();

        Self(jti)
    }
}

impl Default for Jti {
    fn default() -> Self {
        Self::generate()
    }
}

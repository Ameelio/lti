use std::fmt;

use crate::util::OneOrMore;
use crate::util::UtcDateTime;

use serde::{Deserialize, Serialize};

/// These are the standard claims used in OIDC json web tokens that are for
/// identifying the sender, verifying the token is fresh, and protecting from duplication.
#[derive(Clone, Deserialize, Serialize)]
pub struct AuthClaims {
    /// Who the token is intended for, LTI often
    /// passes in the client_id here.
    #[serde(rename = "aud")]
    pub audience: OneOrMore,

    /// Authorized Party - identifies the client
    /// sending the token.
    #[serde(rename = "azp")]
    pub authorized_party: Option<Box<str>>,

    #[serde(rename = "exp")]
    pub expired_at: UtcDateTime,
    #[serde(rename = "iat")]
    pub issued_at: UtcDateTime,

    /// Identifies who created this token.
    #[serde(rename = "iss")]
    pub issuer: Box<str>,

    /// A unique cryptographicly strong string
    /// that helps protect against replay attacks
    /// and duplication.
    pub nonce: Box<str>,
}

impl fmt::Debug for AuthClaims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthClaims")
            .field("expired_at", &self.expired_at)
            .field("issued_at", &self.issued_at)
            .finish_non_exhaustive()
    }
}

impl PartialEq for AuthClaims {
    fn eq(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }
}

impl Eq for AuthClaims {}

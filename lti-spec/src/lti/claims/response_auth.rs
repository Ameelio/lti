use std::fmt;

use chrono::{DateTime, TimeDelta, Utc};
use json_web_tolkien::util::epoch::UtcDateTime;
use serde::{Deserialize, Serialize, ser::SerializeStruct};

/// When sending response JWTs with LTI
/// the client_id is the aud, azp, and iss claim
/// for the token.
#[derive(Clone, Deserialize)]
pub struct ResponseAuth {
    #[serde(rename = "iss")]
    pub client_id: Box<str>,
    #[serde(rename = "aud")]
    pub client_issuer: Box<str>,
    #[serde(rename = "exp")]
    pub expired_at: UtcDateTime,
    #[serde(rename = "iat")]
    pub issued_at: UtcDateTime,
    pub nonce: Box<str>,
}

impl Default for ResponseAuth {
    /// This defaults with the issued_at, expired_at using `Utc::now`, and a randomly generated
    /// nonce via UUID V4.
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();

        let expired_at: UtcDateTime = {
            // the duration of these response tokens should be very short.
            let expired_at: DateTime<Utc> = now.clone() + TimeDelta::minutes(5);

            UtcDateTime(expired_at)
        };

        let issued_at = UtcDateTime(now);

        Self {
            client_id: "".into(),
            client_issuer: "".into(),
            expired_at,
            issued_at,
            nonce: "".into(),
        }
    }
}

impl fmt::Debug for ResponseAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResponseAuth").finish_non_exhaustive()
    }
}

impl PartialEq for ResponseAuth {
    fn eq(&self, other: &Self) -> bool {
        self.nonce == other.nonce
    }
}

impl Eq for ResponseAuth {}

impl Serialize for ResponseAuth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let client_id: &str = self.client_id.as_ref();
        let client_issuer: &str = self.client_issuer.as_ref();

        let mut state = serializer.serialize_struct("ResponseAuth", 6)?;

        state.serialize_field("aud", client_issuer)?;
        state.serialize_field("azp", client_id)?;
        state.serialize_field("exp", &self.expired_at)?;
        state.serialize_field("iat", &self.issued_at)?;
        state.serialize_field("iss", client_id)?;
        state.serialize_field("nonce", &self.nonce)?;
        state.end()
    }
}

use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use serde_jwt::util::epoch::UtcDateTime;
use url::Url;

use crate::oidc::Jti;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestToken {
    pub aud: Box<str>,
    #[serde(rename = "exp")]
    pub expired_at: UtcDateTime,
    #[serde(rename = "iat")]
    pub issued_at: UtcDateTime,
    #[serde(rename = "iss")]
    pub issuer: Box<str>,
    pub jti: Jti,
    pub sub: Box<str>,
}

#[derive(Default)]
pub struct RequestTokenBuilder {
    pub client_identifier: Box<str>,
    pub request_url: Box<str>,
    pub shelf_life: i64,
}

impl RequestToken {
    pub fn builder() -> RequestTokenBuilder {
        RequestTokenBuilder::default()
    }
}

impl RequestTokenBuilder {
    pub fn client_identifier(mut self, value: impl AsRef<str>) -> Self {
        let value: &str = value.as_ref();
        self.client_identifier = value.into();

        self
    }

    pub fn build(self) -> RequestToken {
        let aud = self.request_url;
        let issuer = self.client_identifier.clone();
        let sub = self.client_identifier;

        let now: DateTime<Utc> = Utc::now();
        let issued_at = UtcDateTime(now);
        let expired_at: UtcDateTime = issued_at.clone() + TimeDelta::seconds(self.shelf_life);

        let jti = Jti::generate();

        RequestToken {
            aud,
            expired_at,
            issuer,
            issued_at,
            jti,
            sub,
        }
    }
    pub fn request_url(mut self, value: Url) -> Self {
        self.request_url = value.to_string().into_boxed_str();

        self
    }

    pub fn shelf_life(mut self, value: i64) -> Self {
        self.shelf_life = value;

        self
    }
}

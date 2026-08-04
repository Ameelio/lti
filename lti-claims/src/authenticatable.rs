use json_web_tolkien::jwt::Jwt;
use json_web_tolkien::util::OneOrMore;
use json_web_tolkien::util::epoch::UtcDateTime;
use lti_spec::lti::{DeepLinkingRequest, ResourceLinkRequest, SubmissionReviewRequest};
use serde::{Deserialize, Serialize};

use crate::claims::Claims;

pub trait Authenticatable {
    fn audience(&self) -> &OneOrMore;
    fn authorized_party(&self) -> Option<&str>;
    fn expired_at(&self) -> UtcDateTime;
    fn issued_at(&self) -> UtcDateTime;
    fn issuer(&self) -> &str;
    fn nonce(&self) -> &str;
}

impl<A> Authenticatable for Claims<A>
where
    A: Authenticatable + Clone + for<'de> Deserialize<'de> + Serialize + Send + Sync,
{
    fn audience(&self) -> &OneOrMore {
        self.claims.audience()
    }

    fn authorized_party(&self) -> Option<&str> {
        self.claims.authorized_party()
    }

    fn expired_at(&self) -> UtcDateTime {
        self.claims.expired_at()
    }

    fn issued_at(&self) -> UtcDateTime {
        self.claims.issued_at()
    }

    fn issuer(&self) -> &str {
        self.claims.issuer()
    }

    fn nonce(&self) -> &str {
        self.claims.nonce()
    }
}

impl<C> Authenticatable for DeepLinkingRequest<C> {
    fn audience(&self) -> &OneOrMore {
        &self.auth.audience
    }

    fn authorized_party(&self) -> Option<&str> {
        (self.auth.authorized_party).as_ref().map(AsRef::as_ref)
    }

    fn expired_at(&self) -> UtcDateTime {
        self.auth.expired_at.clone()
    }

    fn issued_at(&self) -> UtcDateTime {
        self.auth.issued_at.clone()
    }

    fn issuer(&self) -> &str {
        self.auth.issuer.as_ref()
    }

    fn nonce(&self) -> &str {
        self.auth.nonce.as_ref()
    }
}

impl<C> Authenticatable for ResourceLinkRequest<C> {
    fn audience(&self) -> &OneOrMore {
        &self.auth.audience
    }

    fn authorized_party(&self) -> Option<&str> {
        (self.auth.authorized_party).as_ref().map(AsRef::as_ref)
    }

    fn expired_at(&self) -> UtcDateTime {
        self.auth.expired_at.clone()
    }

    fn issued_at(&self) -> UtcDateTime {
        self.auth.issued_at.clone()
    }

    fn issuer(&self) -> &str {
        self.auth.issuer.as_ref()
    }

    fn nonce(&self) -> &str {
        self.auth.nonce.as_ref()
    }
}

impl Authenticatable for SubmissionReviewRequest {
    fn audience(&self) -> &OneOrMore {
        &self.auth.audience
    }

    fn authorized_party(&self) -> Option<&str> {
        (self.auth.authorized_party).as_ref().map(AsRef::as_ref)
    }

    fn expired_at(&self) -> UtcDateTime {
        self.auth.expired_at.clone()
    }

    fn issued_at(&self) -> UtcDateTime {
        self.auth.issued_at.clone()
    }

    fn issuer(&self) -> &str {
        self.auth.issuer.as_ref()
    }

    fn nonce(&self) -> &str {
        self.auth.nonce.as_ref()
    }
}

impl<A, Header> Authenticatable for Jwt<A, Header>
where
    A: Authenticatable + for<'de> Deserialize<'de>,
    Header: for<'de> Deserialize<'de>,
{
    fn audience(&self) -> &OneOrMore {
        self.claims().audience()
    }

    fn authorized_party(&self) -> Option<&str> {
        self.claims().authorized_party()
    }

    fn expired_at(&self) -> UtcDateTime {
        self.claims().expired_at()
    }

    fn issued_at(&self) -> UtcDateTime {
        self.claims().issued_at()
    }

    fn issuer(&self) -> &str {
        self.claims().issuer()
    }

    fn nonce(&self) -> &str {
        self.claims().nonce()
    }
}

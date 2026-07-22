pub mod auth_claims;
pub mod error_response;
pub mod jti;
pub mod user_claims;

use serde::{Deserialize, Serialize};

pub use jti::Jti;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationType {
    #[default]
    Web,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenIdClaim {
    Email,
    #[serde(rename = "iss")]
    Issuer,
    FamilyName,
    GivenName,
    Locale,
    Name,
    #[serde(rename = "sub")]
    Subject,
    Picture,
    #[serde(other)]
    Unsupported,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    ClientCredentials,
    Implicit,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GrantTypeList(pub Box<[GrantType]>);

impl Default for GrantTypeList {
    fn default() -> Self {
        let value = [GrantType::ClientCredentials, GrantType::Implicit];
        let value = Box::new(value);

        Self(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    IdToken,
    #[serde(other)]
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResponseTypeList(Box<[ResponseType]>);

impl Default for ResponseTypeList {
    fn default() -> Self {
        let value = [ResponseType::IdToken];
        let value = Box::new(value);

        Self(value)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SigningAlg {
    #[default]
    RS256,
    #[serde(other)]
    Unsupported,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenAuthMethod {
    #[default]
    PrivateKeyJwt,
    #[serde(other)]
    Unsupported,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("your scope lacks permissions to use this endpoint")]
    AgsNotPermitted,
    #[error("Domain is not found in url")]
    InvalidDomain,
    #[error("Unable to parse domain: {source}")]
    DomainParseError { source: url::ParseError },
    #[error(transparent)]
    JsonError {
        #[from]
        source: serde_json::Error,
    },
    #[error("the ags url is invalid")]
    InvalidAgsUrl,
    #[error("Invalid OIDC configuration: {source}")]
    InvalidOidcConfiguration { source: serde_json::Error },
    #[error("Inavlid Registration Request {source}")]
    InvalidRegistrationRequest { source: serde_json::Error },
    #[error("Invalid Registration, {source}")]
    InvalidRegistration { source: serde_json::Error },
    #[error("Inavlid Registration Response {source}")]
    InvalidRegistrationResponse { source: serde_json::Error },
    #[error("this endpoint has no line item url")]
    MissingAgsLineItem,
    #[error("unexpected error encoding/decoding query strings, {source}")]
    UnexpectedQsError {
        #[from]
        source: serde_qs::Error,
    },
    #[error("unexpected error when trying to set url scheme")]
    UnexpectedUrlSchemeError,
}

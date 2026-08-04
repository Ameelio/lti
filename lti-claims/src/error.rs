use axum::extract::rejection::FormRejection;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use lti_spec::oidc::error_response::ErrorResponse;
use thiserror::Error;

// Todo: Add a spec struct for error response, then we
// can just put that in as an extension.
#[derive(Debug, Error)]
pub enum Error {
    #[error("azp claim is required to be in aud claim")]
    AzpNotInAud,
    #[error("client is not in audience")]
    ClientNotInAud,
    #[error("client is not an authorized party")]
    ClientNotAnAzp,
    #[error("client configuration has no client_id")]
    ClientConfigurationMissingClientId,
    #[error("duplicate request")]
    DuplicateRequest,
    #[error("unable to decode form, {source}")]
    FormRejection {
        #[from]
        source: FormRejection,
    },
    #[error(transparent)]
    Infallible {
        #[from]
        source: std::convert::Infallible,
    },
    #[error("csrf cookie does not match")]
    InvalidCsrfCookie,
    #[error("invalid issuer url, {source}")]
    InvalidIssuerUrl { source: url::ParseError },
    #[error("invalid jwks url, {source}")]
    InvalidJwksUrl { source: url::ParseError },
    #[error("expiration is an invalid datetime")]
    InvalidTokenExpiration,
    #[error("signature is invalid, {source}")]
    InvalidSignature { source: Box<dyn std::error::Error> },
    #[error("issued at timestamp is in the future")]
    IssuedAtInFuture { issued_at: Box<str> },
    #[error("token expired at: {expired_at}")]
    ExpiredToken { expired_at: Box<str> },
    #[error("form body is malformed")]
    MalformedBody,
    #[error("ClaimsLayer is missing, please add it to the router")]
    MissingClaimsLayer,
    #[error("client configuration missing")]
    MissingClientConfiguration,
    #[error("missing csrf cookie")]
    MissingCsrfCookie,
    #[error("required either single aud or an azp claim")]
    NoDiscreteClientId,
    #[error("unable to fetch jwk")]
    UnableToFetchJwk,
    #[error("unable to parse json web key, {source}")]
    UnableToParseJwk {
        #[from]
        source: serde_json::Error,
    },
    #[error("unable to parse json web token, {source}")]
    UnableToParseJwt {
        #[from]
        source: json_web_tolkien::error::Error,
    },
    #[error("an unexpected error occured when fetching from cache, {source}")]
    UnexpectedCacheError {
        #[from]
        source: Box<dyn std::error::Error>,
    },
    #[error("an unexpected error occured when making http request, {source}: {}", std::error::Error::source(.source).map(|e| e.to_string()).unwrap_or_default())]
    UnexpectedHttpError {
        #[from]
        source: reqwest::Error,
    },
}

impl Error {
    fn code(&self) -> &'static str {
        match self {
            Self::DuplicateRequest => "conflict",
            Self::FormRejection { source: _ } | Self::InvalidTokenExpiration => "invalid_request",
            Self::ExpiredToken { expired_at: _ } | Self::InvalidSignature { source: _ } => {
                "unauthorized"
            }
            Self::MissingClientConfiguration => "invalid_client_configuration",
            Self::MissingCsrfCookie | Self::InvalidCsrfCookie => "forbidden",
            _ => "internal_server_error",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::DuplicateRequest => "Duplicate Request.",
            Self::FormRejection { source: _ } | Self::InvalidTokenExpiration => "Invalid Request.",
            Self::ExpiredToken { expired_at: _ } | Self::InvalidSignature { source: _ } => {
                "Unauthorized Request."
            }
            Self::MissingClientConfiguration => "Invalid Client Configuration.",
            Self::MissingCsrfCookie | Self::InvalidCsrfCookie => "Forbidden Request.",
            _ => "Unexpected Error.",
        }
    }

    fn errors(&self) -> Box<[Box<str>]> {
        let message: Box<str> = self.to_string().into();

        Box::new([message])
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::DuplicateRequest => StatusCode::CONFLICT,
            Self::FormRejection { source: _ }
            | Self::InvalidTokenExpiration
            | Self::MissingClientConfiguration => StatusCode::BAD_REQUEST,
            Self::ExpiredToken { expired_at: _ } | Self::InvalidSignature { source: _ } => {
                StatusCode::UNAUTHORIZED
            }
            Self::MissingCsrfCookie | Self::InvalidCsrfCookie => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let code: Box<str> = self.code().into();
        let description: Box<str> = self.description().into();
        let errors: Option<Box<[Box<str>]>> = Some(self.errors());

        let er = ErrorResponse {
            code,
            description,
            errors,
        };

        let mut response = (self.status_code()).into_response();

        response.extensions_mut().insert(er);

        response
    }
}

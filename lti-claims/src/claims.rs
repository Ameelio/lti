use std::borrow::Cow;
use std::fmt;

use axum::Form;
use axum::body::Body;
use axum::extract::{FromRequest, Request};
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use serde_jwt::prelude::*;
use tracing::instrument;

use crate::CsrfResult;
use crate::authenticatable::Authenticatable;
use crate::claims_state::ClaimsState;
use crate::error::Error;
use crate::fetch_client::fetch_client;
use crate::lms_client::LmsClient;
use crate::protect_against_duplication::protect_against_duplication;
use crate::validate_claims::validate_claims;
use crate::verify_signature::verify_signature;

pub(crate) type IdToken<T> = Jwt<T, Header>;

#[derive(Clone)]
pub struct Claims<T: Authenticatable + Clone + for<'de> Deserialize<'de> + Serialize + Send + Sync>
{
    pub claims: T,
    pub client: LmsClient,
}

impl<T> fmt::Debug for Claims<T>
where
    T: Authenticatable + Clone + for<'de> Deserialize<'de> + fmt::Debug + Serialize + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Claims")
            .field("claims", &self.claims)
            .field("client", &self.client)
            .finish()
    }
}

impl<A, S> FromRequest<S> for Claims<A>
where
    A: Authenticatable + Clone + for<'de> Deserialize<'de> + Serialize + Send + Sync,
    S: ClaimsState + Send + Sync,
{
    type Rejection = Error;

    #[instrument(err)]
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body): (Parts, Body) = req.into_parts();

        let Form(form): Form<FormData> = {
            let req = Request::from_parts(parts.clone(), body);

            Form::from_request(req, state).await?
        };

        let jws: Jws = form.jws;

        let id_token: IdToken<A> = {
            let id_token: Cow<str> = jws.encoded_token();

            IdToken::try_from(id_token.as_ref())?
        };

        match state
            .protect_against_forgery(&form.actual_fat, &mut parts)
            .await
        {
            CsrfResult::Ok => {}
            CsrfResult::Invalid => {
                return Err(Error::InvalidCsrfCookie);
            }
            CsrfResult::Missing => {
                return Err(Error::MissingCsrfCookie);
            }
        };

        let client: LmsClient = fetch_client(state, &id_token)?;

        validate_claims(&id_token, &client)?;
        protect_against_duplication(state, &id_token).await?;
        verify_signature(state, &client, jws).await?;

        {
            let claims: &A = id_token.claims();

            let claims: A = claims.to_owned();

            let claims = Claims { claims, client };

            Ok(claims)
        }
    }
}

#[derive(Deserialize)]
struct FormData {
    #[serde(rename = "state")]
    actual_fat: String,
    #[serde(rename = "id_token")]
    jws: Jws,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Header {
    pub alg: Algorithm,
    pub kid: Box<str>,
}

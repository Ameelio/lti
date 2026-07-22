use axum::extract::Form;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use lti_spec::lti::{AuthRequestUrl, LaunchForm};
use lti_spec::oidc::error_response::ErrorResponse;
use lti_spec::util::UrlSet;
use thiserror::Error;
use tracing::instrument;
use url::Url;

use crate::LaunchCsrf;
use crate::{LaunchClient, LaunchState};

#[instrument(err)]
pub async fn post_launch_handler<S>(
    jar: CookieJar,
    State(state): State<S>,
    Form(form): Form<LaunchForm>,
) -> Result<(CookieJar, Redirect), Error<S>>
where
    S: LaunchState,
{
    let csrf: LaunchCsrf = state.csrf();

    let redirect: Redirect = {
        let client: LaunchClient = state
            .fetch_client(&form.client_id, &form.issuer)
            .await
            .map_err(|source| Error::UnableToFetchClient { source })?;

        let redirect_url: Url = {
            let redirect_urls: &UrlSet = state.redirect_urls();
            let url: Url = form.target_link_uri.ok_or(Error::TargetLinkUrlIsMissing)?;

            if redirect_urls.contains(&url) {
                url
            } else {
                return Err(Error::TargetLinkUrlIsUnauthorized {
                    url: url.to_string(),
                });
            }
        };

        let login_hint_some: bool = form.login_hint.is_some();
        let lti_msg_some: bool = form.lti_message_hint.is_some();

        let login_hint: String = form.login_hint.unwrap_or_default();
        let lti_message_hint: String = form.lti_message_hint.unwrap_or_default();

        let mut builder = AuthRequestUrl::builder(
            client.auth_url,
            client.client_identifier.clone(),
            redirect_url,
        )
        .csrf(csrf.str.as_ref())
        .add_to_nonce(client.client_identifier)
        .add_to_nonce(csrf.str.as_ref())
        .add_to_nonce(login_hint.as_str());

        if login_hint_some {
            builder = builder.login_hint(login_hint);
        }

        if lti_msg_some {
            builder = builder.lti_message_hint(lti_message_hint);
        }

        let url = builder
            .build()
            .map_err(|source| Error::UnableToBuildRedirect { source })?;

        let url: &str = url.as_ref();

        Redirect::to(url)
    };
    let updated_jar = jar.add(csrf.cookie);

    Ok((updated_jar, redirect))
}

#[derive(Debug, Error)]
pub enum Error<S: LaunchState> {
    #[error("unable to fetch client, {source}")]
    UnableToFetchClient { source: S::Error },
    #[error("unable to build redirect, {source}")]
    UnableToBuildRedirect { source: lti_spec::error::Error },
    #[error("target_link_url is required")]
    TargetLinkUrlIsMissing,
    #[error("target_link_url, {url} is not authorized")]
    TargetLinkUrlIsUnauthorized { url: String },
}

impl<S> IntoResponse for Error<S>
where
    S: LaunchState,
{
    fn into_response(self) -> axum::response::Response {
        let error: String = self.to_string();

        match self {
            Self::TargetLinkUrlIsMissing | Self::TargetLinkUrlIsUnauthorized { url: _ } => {
                let mut resp = (StatusCode::UNAUTHORIZED).into_response();

                let err = ErrorResponse::unauthorized("Unauthorized");

                resp.extensions_mut().insert(err);

                resp
            }
            _ => {
                let mut resp = (StatusCode::INTERNAL_SERVER_ERROR).into_response();

                let err = ErrorResponse::internal("Unexpected Error", &error);

                resp.extensions_mut().insert(err);

                resp
            }
        }
    }
}

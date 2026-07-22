use std::fmt;
use std::future::Future;

use axum_extra::extract::cookie::Cookie;
use lti_spec::util::UrlSet;
use url::Url;

pub trait LaunchState: Clone + fmt::Debug {
    type Error: std::error::Error + Send;
    fn csrf(&self) -> LaunchCsrf;

    fn fetch_client(
        &self,
        client_id: impl AsRef<str>,
        issuer: impl AsRef<str>,
    ) -> impl Future<Output = Result<LaunchClient, Self::Error>>;

    fn redirect_urls(&self) -> &UrlSet;
}

pub struct LaunchCsrf {
    pub cookie: Cookie<'static>,
    pub str: Box<str>,
}

pub struct LaunchClient {
    pub auth_url: Url,
    pub client_identifier: Box<str>,
}

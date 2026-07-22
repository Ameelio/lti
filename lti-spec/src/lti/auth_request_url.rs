use std::hash::{DefaultHasher, Hash, Hasher};

use serde::Serialize;
use url::Url;

use crate::error::Error;

/// A url used for redirects within the LTI Oauth workflow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthRequestUrl(pub Url);

pub struct AuthRequestUrlBuilder {
    auth_url: Url,
    data: FormData,
    nonce_components: Vec<Box<str>>,
}

impl AuthRequestUrl {
    pub fn builder(auth_url: Url, client_id: Box<str>, redirect_uri: Url) -> AuthRequestUrlBuilder {
        let data = FormData {
            client_id,
            redirect_uri,
            ..FormData::default()
        };

        let nonce_components: Vec<Box<str>> = Vec::with_capacity(3);

        AuthRequestUrlBuilder {
            auth_url,
            data,
            nonce_components,
        }
    }
}

impl AsRef<str> for AuthRequestUrl {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl AuthRequestUrlBuilder {
    pub fn build(self) -> Result<AuthRequestUrl, Error> {
        let mut auth_url = self.auth_url;

        let mut data = self.data;

        let nonce: String = {
            let mut hasher = DefaultHasher::new();

            self.nonce_components.hash(&mut hasher);

            let nonce: u64 = hasher.finish();

            format!("{nonce}")
        };

        data.nonce = nonce.into_boxed_str();

        let data = serde_qs::to_string(&data)?;
        let data = Some(data.as_str());

        auth_url.set_query(data);

        let req = AuthRequestUrl(auth_url);

        Ok(req)
    }

    pub fn add_to_nonce<T>(mut self, value: T) -> Self
    where
        T: Into<Box<str>>,
    {
        self.nonce_components.push(value.into());

        self
    }
    pub fn csrf<T>(mut self, csrf: T) -> Self
    where
        T: Into<Box<str>>,
    {
        self.data.csrf = Some(csrf.into());

        self
    }

    pub fn lti_message_hint<T>(mut self, hint: T) -> Self
    where
        T: Into<Box<str>>,
    {
        self.data.lti_message_hint = Some(hint.into());

        self
    }

    pub fn login_hint<T>(mut self, hint: T) -> Self
    where
        T: Into<Box<str>>,
    {
        self.data.login_hint = Some(hint.into());

        self
    }
}

#[derive(Serialize)]
pub struct FormData {
    client_id: Box<str>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    csrf: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lti_message_hint: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_hint: Option<Box<str>>,
    nonce: Box<str>,
    prompt: Prompt,
    redirect_uri: Url,
    response_mode: ResponseMode,
    response_type: ResponseType,
    scope: Scope,
}

impl Default for FormData {
    fn default() -> Self {
        let redirect_uri = Url::parse("https://example.com").unwrap();

        Self {
            client_id: Box::default(),
            csrf: Option::None,
            lti_message_hint: Option::default(),
            login_hint: Option::default(),
            nonce: Box::default(),
            prompt: Prompt::default(),
            redirect_uri,
            response_mode: ResponseMode::default(),
            response_type: ResponseType::default(),
            scope: Scope::default(),
        }
    }
}
#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
enum ResponseMode {
    #[default]
    FormPost,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
enum ResponseType {
    #[default]
    IdToken,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
enum Prompt {
    #[default]
    None,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
enum Scope {
    #[default]
    Openid,
}

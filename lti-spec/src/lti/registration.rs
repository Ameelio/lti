use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use url::Url;

use super::ToolConfiguration;

use crate::domain::Domain;
use crate::error::Error;
use crate::lti::claims::canvas_privacy_level::CanvasPrivacyLevel;
use crate::lti::claims::prelude::CustomParams;
use crate::lti::{MessageConfig, Scope, ScopeList, ToolConfigurationOptions};
use crate::oidc::user_claims::SupportedUserClaims;
use crate::oidc::*;

pub trait Registerable {
    fn accepted_messages(&self) -> Vec<MessageConfig>;
    fn allowed_redirects(&self) -> Vec<Url>;
    fn required_claims(&self) -> Vec<SupportedUserClaims>;
    fn requred_scopes(&self) -> Vec<Scope>;
}

/// A Registration Request in JSON form.
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct RegistrationRequestJson(pub serde_json::Value);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistrationRequest {
    pub client_name: Box<str>,
    pub initiate_login_uri: Url,
    pub jwks_uri: Url,

    #[serde(rename = "https://purl.imsglobal.org/spec/lti-tool-configuration")]
    pub lti_tool_configuration: ToolConfiguration,

    #[serde(default, flatten)]
    pub options: RegistrationRequestOptions,

    pub redirect_uris: Box<[Url]>,
}

pub struct RegistrationRequestBuilder<'a> {
    claims: HashSet<SupportedUserClaims>,
    client_name: &'a str,
    contacts: Vec<&'a str>,
    host: Url,
    initiate_login_path: &'a str,
    jwks_path: &'a str,
    messages: Vec<MessageConfig>,
    options: RegistrationRequestOptions,
    redirect_uris: Vec<Url>,
    scopes: HashSet<Scope>,
    target_link_path: &'a str,
    tool_config_options: ToolConfigurationOptions,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegistrationRequestOptions {
    #[serde(default)]
    pub application_type: ApplicationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Box<[Box<str>]>>,
    #[serde(default)]
    pub grant_types: GrantTypeList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<Url>,
    #[serde(default)]
    pub response_types: ResponseTypeList,
    #[serde(default)]
    pub token_endpoint_auth_method: TokenAuthMethod,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tos_uri: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_uri: Option<Url>,
    #[serde(default)]
    pub scope: Option<ScopeList>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct RegistrationResponse {
    pub client_id: String,
}

impl RegistrationRequest {
    pub fn builder<'a>(host: Url) -> RegistrationRequestBuilder<'a> {
        let jwks_path = "/.well-known/jwks.json";

        let tool_config_options = ToolConfigurationOptions::default();

        let claims: HashSet<SupportedUserClaims> =
            tool_config_options.claims.clone().into_iter().collect();

        RegistrationRequestBuilder {
            claims,
            client_name: "",
            contacts: Vec::new(),
            host,
            initiate_login_path: "",
            jwks_path,
            messages: Vec::new(),
            options: RegistrationRequestOptions::default(),
            redirect_uris: Vec::new(),
            scopes: HashSet::new(),
            target_link_path: "",
            tool_config_options: ToolConfigurationOptions::default(),
        }
    }
}

impl RegistrationRequestJson {
    pub fn from_registration(value: RegistrationRequest) -> Result<Self, Error> {
        let inner: serde_json::Value =
            serde_json::to_value(value).map_err(|source| Error::InvalidRegistration { source })?;

        let rrj = Self(inner);

        Ok(rrj)
    }
}

impl<'a> RegistrationRequestBuilder<'a> {
    pub fn add_contact(mut self, value: &'a str) -> Self {
        self.contacts.push(value);

        self
    }

    pub fn add_custom_param(
        mut self,
        name: impl Into<Box<str>>,
        mapping: impl Into<Box<str>>,
    ) -> Self {
        let mut params: CustomParams = self
            .tool_config_options
            .custom_parameters
            .unwrap_or_default();

        params.insert(name.into(), mapping.into());

        self.tool_config_options.custom_parameters = Some(params);

        self
    }

    pub fn build(self) -> Result<RegistrationRequest, Error> {
        let client_name: Box<str> = self.client_name.into();

        let initiate_login_uri = self.set_url(self.initiate_login_path);
        let jwks_uri = self.set_url(self.jwks_path);
        let target_link_uri: Url = self.set_url(self.target_link_path);

        let lti_tool_configuration: ToolConfiguration = {
            let claims: Vec<SupportedUserClaims> = self.claims.into_iter().collect();

            let domain = Domain::try_from(self.host.clone())?;

            let messages: Box<[MessageConfig]> = self.messages.into_boxed_slice();

            let mut options: ToolConfigurationOptions = self.tool_config_options;

            options.claims = claims.into_boxed_slice();

            let target_link_uri = target_link_uri.clone();

            ToolConfiguration {
                domain,
                messages,
                options,
                target_link_uri,
            }
        };

        let mut options: RegistrationRequestOptions = self.options;

        if self.contacts.len() > 0 {
            let contacts: Vec<Box<str>> = self.contacts.into_iter().map(Box::from).collect();
            let contacts: Box<[Box<str>]> = contacts.into_boxed_slice();

            options.contacts = Some(contacts);
        }

        if self.scopes.len() > 0 {
            let scopes: Vec<Scope> = self.scopes.into_iter().collect();
            let scopes: Box<[Scope]> = scopes.into_boxed_slice();
            let scopes = ScopeList(scopes);

            options.scope = Some(scopes)
        }

        let redirect_uris: Box<[Url]> = {
            let mut redirect_uris: Vec<Url> = self.redirect_uris;

            redirect_uris.push(target_link_uri);

            redirect_uris.into_boxed_slice()
        };

        let reg = RegistrationRequest {
            client_name,
            initiate_login_uri,
            jwks_uri,
            lti_tool_configuration,
            options,
            redirect_uris,
        };

        Ok(reg)
    }

    pub fn client_name(mut self, value: &'a str) -> Self {
        self.client_name = value;
        self
    }

    pub fn client_path(mut self, value: &str) -> Self {
        let client_uri: Url = self.set_url(value);
        let client_uri = Some(client_uri);

        self.options.client_uri = client_uri;

        self
    }

    pub fn description(mut self, value: &'a str) -> Self {
        let value: Box<str> = value.into();

        self.tool_config_options.description = Some(value);

        self
    }

    pub fn host(mut self, value: Url) -> Self {
        self.host = value;
        self
    }

    pub fn initiate_login_path(mut self, value: &'a str) -> Self {
        self.initiate_login_path = value;
        self
    }

    pub fn jwks_path(mut self, value: &'a str) -> Self {
        self.jwks_path = value;
        self
    }

    pub fn logo_path(mut self, value: &str) -> Self {
        let logo_uri: Url = self.set_url(value);
        let logo_uri = Some(logo_uri);

        self.options.logo_uri = logo_uri;

        self
    }

    pub fn policy_path(mut self, value: &str) -> Self {
        let policy_uri: Url = self.set_url(value);
        let policy_uri = Some(policy_uri);

        self.options.policy_uri = policy_uri;

        self
    }

    pub fn privacy_level_anonymous(mut self) -> Self {
        self.tool_config_options.privacy_level = CanvasPrivacyLevel::Anonymous;

        self
    }

    pub fn privacy_level_email_only(mut self) -> Self {
        self.tool_config_options.privacy_level = CanvasPrivacyLevel::EmailOnly;

        self
    }

    pub fn privacy_level_name_only(mut self) -> Self {
        self.tool_config_options.privacy_level = CanvasPrivacyLevel::NameOnly;

        self
    }

    pub fn privacy_level_public(mut self) -> Self {
        self.tool_config_options.privacy_level = CanvasPrivacyLevel::Public;

        self
    }

    pub fn register<R>(mut self, value: &R) -> Self
    where
        R: Registerable,
    {
        let claims: Vec<SupportedUserClaims> = value.required_claims();
        let mut messages: Vec<MessageConfig> = value.accepted_messages();
        let scopes: Vec<Scope> = value.requred_scopes();
        let mut redirects: Vec<Url> = value.allowed_redirects();

        self.claims.extend(claims);
        self.messages.append(&mut messages);
        self.scopes.extend(scopes);
        self.redirect_uris.append(&mut redirects);

        self
    }

    pub fn target_link_path(mut self, value: &'a str) -> Self {
        self.target_link_path = value;

        self
    }

    pub fn tos_path(mut self, value: &str) -> Self {
        let tos_uri: Url = self.set_url(value);
        let tos_uri = Some(tos_uri);

        self.options.tos_uri = tos_uri;

        self
    }

    fn set_url(&self, path: &str) -> Url {
        let mut url = self.host.clone();
        url.set_path(path);

        url
    }
}

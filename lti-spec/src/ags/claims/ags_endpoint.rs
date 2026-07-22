use serde::{Deserialize, Serialize};
use url::Url;

use super::ags_scope::AgsScope;
use crate::error::Error;

/// Provides an endpoint the provider can use to access the platform's line items (grades)
/// See [LTI Assignment and Grades Specification](https://www.imsglobal.org/spec/lti-ags/v2p0)
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum AgsEndpoint {
    Permitted(AgsPermittedEndpoint),
    Restricted(AgsRestrictedEndpoint),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AgsPermittedEndpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lineitem: Option<Url>,
    pub lineitems: Url,
    pub scope: Box<[AgsScope]>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AgsRestrictedEndpoint {
    pub scope: Box<[AgsScope]>,
}

impl AgsEndpoint {
    pub fn lineitem(&self, path: Option<&str>) -> Result<Url, Error> {
        let mut url = match self {
            Self::Permitted(v) => v.lineitem.clone().ok_or(Error::MissingAgsLineItem)?,
            _ => return Err(Error::AgsNotPermitted),
        };

        url.set_scheme("https")
            .map_err(|_| Error::UnexpectedUrlSchemeError)?;

        if let Some(path) = path {
            url.path_segments_mut()
                .map_err(|_| Error::InvalidAgsUrl)?
                .push(path);
        }

        Ok(url)
    }
}

use std::fmt;

use oxilangtag::LanguageTag;
use serde::{Deserialize, Serialize};

/// OIDC claims that identify the user
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct UserClaims {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<str>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<Box<str>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<Box<str>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<LanguageTag<Box<str>>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,

    #[serde(rename = "sub")]
    pub user_id: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedUserClaims {
    Email,
    FamilyName,
    GiveName,
    Locale,
    Name,
    #[serde(rename = "sub")]
    Subject,
}

impl fmt::Debug for UserClaims {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserClaims").finish_non_exhaustive()
    }
}

impl PartialEq for UserClaims {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Eq for UserClaims {}

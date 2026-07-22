use serde::{Deserialize, Serialize};

use crate::lti::claims::role::Role;

/// Claim that provides whom this sumbission the user (sub claim) is owned by.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForUser {
    #[serde(flatten)]
    pub options: ForUserOptions,
    pub user_id: Box<str>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ForUserOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_sourceid: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Box<[Role]>>,
}

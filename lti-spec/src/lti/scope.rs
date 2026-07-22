use std::fmt;

use serde::{Deserialize, Serialize, de};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopeList(pub Box<[Scope]>);

pub struct ScopeListBuilder(Vec<Scope>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Scope {
    Assignments,
    AssignmentsReadOnly,
    NoticeHandlers,
    ResultReadOnly,
    RosterReadOnly,
    Score,
    Other(Box<str>),
}

impl ScopeList {
    pub fn builder() -> ScopeListBuilder {
        ScopeListBuilder(Vec::with_capacity(2))
    }
}

impl<'de> Deserialize<'de> for ScopeList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Visitor)
    }
}

impl Serialize for ScopeList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let scopes: Vec<String> = self
            .0
            .as_ref()
            .iter()
            .map(|x: &Scope| x.to_string())
            .collect();
        let scopes: String = scopes.join(" ");

        serializer.serialize_str(&scopes)
    }
}

impl ScopeListBuilder {
    pub fn add_scope(mut self, scope: Scope) -> Self {
        self.0.push(scope);
        self
    }

    pub fn build(self) -> ScopeList {
        let list: Box<[Scope]> = self.0.into_boxed_slice();

        ScopeList(list)
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assignments => {
                write!(f, "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem")
            }
            Self::AssignmentsReadOnly => write!(
                f,
                "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem.readonly"
            ),
            Self::NoticeHandlers => write!(
                f,
                "https://purl.imsglobal.org/spec/lti/scope/noticehandlers"
            ),
            Self::ResultReadOnly => write!(
                f,
                "https://purl.imsglobal.org/spec/lti-ags/scope/result.readonly"
            ),
            Self::RosterReadOnly => write!(
                f,
                "https://purl.imsglobal.org/spec/lti-nrps/scope/contextmembership.readonly"
            ),
            Self::Score => write!(f, "https://purl.imsglobal.org/spec/lti-ags/scope/score"),
            Self::Other(x) => write!(f, "{}", x),
        }
    }
}

impl From<&str> for Scope {
    fn from(value: &str) -> Self {
        match value {
            "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem" => Self::Assignments,
            "https://purl.imsglobal.org/spec/lti-ags/scope/lineitem.readonly" => {
                Self::AssignmentsReadOnly
            }
            "https://purl.imsglobal.org/spec/lti/scope/noticehandlers" => Self::NoticeHandlers,
            "https://purl.imsglobal.org/spec/lti-ags/scope/result.readonly" => Self::ResultReadOnly,
            "https://purl.imsglobal.org/spec/lti-nrps/scope/contextmembership.readonly" => {
                Self::RosterReadOnly
            }
            "https://purl.imsglobal.org/spec/lti-ags/scope/score" => Self::Score,
            other => Self::Other(other.into()),
        }
    }
}

struct Visitor;

impl<'de> de::Visitor<'de> for Visitor {
    type Value = ScopeList;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A space seperated list of scopes")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let scopes: Vec<&str> = v.split(" ").map(|x: &str| x.trim()).collect();
        let scopes: Vec<Scope> = scopes.into_iter().map(Scope::from).collect();
        let scopes: Box<[Scope]> = scopes.into_boxed_slice();

        Ok(ScopeList(scopes))
    }
}

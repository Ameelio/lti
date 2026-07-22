use serde::{Deserialize, Serialize};

/// An LTI Role, this enum focuses on the core roles, other roles will be serialized into Other.
/// See [LTI Spec - Role Volcabularies](https://www.imsglobal.org/spec/lti/v1p3/#role-vocabularies)
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Role {
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/membership#Administrator")]
    ContextAdministrator,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/membership#ContentDeveloper")]
    ContextDeveloper,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/membership#Instructor")]
    ContextInstructor,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/membership#Learner")]
    ContextLearner,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/membership#Mentor")]
    ContextMentor,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Administrator")]
    InstitutionAdministrator,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Faculty")]
    InstitutionFaculty,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Guest")]
    InstitutionGuest,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/institution/person#None")]
    InstutionNone,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Other")]
    InstitutionOther,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Staff")]
    InstitutionStaff,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/institution/person#Student")]
    InstitutionStudent,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/system/person#Administrator")]
    SystemAdministrator,
    #[serde(rename = "http://purl.imsglobal.org/vocab/lis/v2/system/person#None")]
    SystemNone,
    #[serde(untagged)]
    Other(Box<str>),
}

impl PartialEq<Role> for &Role {
    fn eq(&self, other: &Role) -> bool {
        self == other
    }
}

use serde::Serialize;

use crate::lti::{Scope, ScopeList, scope::ScopeListBuilder};

/// Request Form to obtain an access token per LTI Spec.
/// ## Example
///   use lti_spec::access_request;
///   use lti_spec::lti::Scope;
///
///   let client_assertion = "jws";
///
///   let form = access_request::RequestForm::builder(&client_assertion)
///     .can_onlt_read_results()
///     .can_only_read_roster()
///     .can_write_assignments()
///     .can_write_notice_handlers()
///     .can_write_scores()
///
#[derive(Serialize)]
pub struct RequestForm {
    pub grant_type: GrantType,
    pub client_assertion_type: ClientAssertionType,
    pub client_assertion: Box<str>,
    pub scope: ScopeList,
}

pub struct RequestFormBuilder {
    client_assertion: Box<str>,
    scope: ScopeListBuilder,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientAssertionType {
    #[default]
    #[serde(rename = "urn:ietf:params:oauth:client-assertion-type:jwt-bearer")]
    JwtBearer,
}
#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    #[default]
    ClientCredentials,
}

impl RequestForm {
    pub fn builder(client_assertion: Box<str>) -> RequestFormBuilder {
        RequestFormBuilder {
            client_assertion,
            scope: ScopeList::builder(),
        }
    }
}

impl RequestFormBuilder {
    pub fn add_scope(mut self, scope: Scope) -> Self {
        self.scope = self.scope.add_scope(scope);

        self
    }

    pub fn can_only_read_assignments(self) -> Self {
        self.add_scope(Scope::AssignmentsReadOnly)
    }

    pub fn can_onlt_read_results(self) -> Self {
        self.add_scope(Scope::ResultReadOnly)
    }

    pub fn can_only_read_roster(self) -> Self {
        self.add_scope(Scope::RosterReadOnly)
    }
    pub fn can_write_assignments(self) -> Self {
        self.add_scope(Scope::Assignments)
    }

    pub fn can_write_notice_handlers(self) -> Self {
        self.add_scope(Scope::NoticeHandlers)
    }

    pub fn can_write_scores(self) -> Self {
        self.add_scope(Scope::Score)
    }

    pub fn build(self) -> RequestForm {
        let scope = self.scope.build();

        RequestForm {
            client_assertion: self.client_assertion,
            client_assertion_type: ClientAssertionType::default(),
            grant_type: GrantType::default(),
            scope,
        }
    }
}

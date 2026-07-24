use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Custom Parameters that can be passed between the Lms Client to the Tool.
// See [Lti Message Spec](https://www.imsglobal.org/spec/lti-dr/v1p0#lti-message)
// See [Custom Parameter Substitution](https://www.imsglobal.org/spec/lti/v1p3#customproperty)
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomParams(pub HashMap<Box<str>, Box<str>>);

impl CustomParams {
    pub fn get<'a>(&'a self, key: impl Into<Box<str>>) -> Option<&'a str> {
        match self.0.get(&key.into()) {
            Some(v) => Some(v.as_ref()),
            None => None,
        }
    }

    pub fn insert(
        &mut self,
        key: impl Into<Box<str>>,
        value: impl Into<Box<str>>,
    ) -> Option<Box<str>> {
        self.0.insert(key.into(), value.into())
    }
}

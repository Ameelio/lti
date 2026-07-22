use serde::{Deserialize, Serialize};

/// Determines what PII data Canvas will share with the tool.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanvasPrivacyLevel {
    #[default]
    Anonymous,
    EmailOnly,
    NameOnly,
    Public,
}

impl CanvasPrivacyLevel {
    pub const fn is_anonymous(&self) -> bool {
        matches!(*self, Self::Anonymous)
    }
}

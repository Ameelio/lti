use serde::{Deserialize, Serialize};

// TODO: Put actual santiization helpers here.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SanitizedHtml(pub Box<str>);

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Version {
    #[serde(rename = "1.3.0")]
    #[default]
    V130,
}

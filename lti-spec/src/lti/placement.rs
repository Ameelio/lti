use serde::{Deserialize, Serialize};

use crate::canvas::CanvasPlacement;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Placement {
    Canvas(CanvasPlacement),
    Other(Box<str>),
}

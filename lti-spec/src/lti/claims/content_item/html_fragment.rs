use serde::{Deserialize, Serialize};

use crate::util::SanitizedHtml;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HtmlFragment {
    /// html content, must be pre-sanitized.
    pub html: SanitizedHtml,
    /// plain text description of the content item.
    pub text: Box<str>,
    pub title: Box<str>,
}

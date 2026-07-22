use rust_decimal::Decimal;
use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AgsLineItemsResponse(pub Box<[AgsLineItem]>);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgsLineItem {
    pub id: Url,
    pub label: Box<str>,
    pub resource_id: Box<str>,
    pub resource_link_id: Box<str>,
    pub score_maximum: Decimal,
    #[serde(rename = "https://canvas.instructure.com/lti/submission_type")]
    pub submission_type: SubmissionType,
    pub tag: Box<str>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SubmissionType {
    #[serde(rename = "type")]
    submission_type: Box<str>,
    external_tool_url: Url,
}

impl AsRef<[AgsLineItem]> for AgsLineItemsResponse {
    fn as_ref(&self) -> &[AgsLineItem] {
        self.0.as_ref()
    }
}

impl<'a> IntoIterator for &'a AgsLineItemsResponse {
    type Item = &'a AgsLineItem;
    type IntoIter = std::slice::Iter<'a, AgsLineItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.as_ref().into_iter()
    }
}

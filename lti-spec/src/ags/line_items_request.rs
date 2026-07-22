use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct AgsLineItemsReqParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_link_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<&'a str>,
}

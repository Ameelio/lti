use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::lti::claims::prelude::CustomParams;
/// A Line item in a gradebook.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LineItem {
    /// if set to true, tells the platform to release the grades
    /// to learners
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gradesReleased")]
    pub grades_released: Option<bool>,

    /// label for the line item, otherwise title will be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<Box<str>>,

    #[serde(default, skip_serializing_if = "Decimal::is_zero")]
    #[serde(rename = "scoreMaximum")]
    pub score_max: Decimal,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "submissionReview")]
    pub submission_review: Option<SubmissionReview>,

    /// Used by the tool/etc for additional metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<Box<str>>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubmissionReview {
    /// Custom parameters to send to the tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomParams>,
    /// Allows the tool to decorate the link with a label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,
    /// The url to submit a submission review request to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
}

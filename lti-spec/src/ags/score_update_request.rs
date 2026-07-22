use std::ops::Not;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use url::Url;

/// Request to update the score of a line_item on the platform.
/// See [OpenAPI Spec](https://www.imsglobal.org/spec/lti-ags/v2p0/openapi/#/default/Scores.POST)
/// See [Canvas API](https://developerdocs.instructure.com/services/canvas/resources/score)
/// ## Example
///     use chrono::{DateTime, Utc};
///     use lti_spec::prelude::*;
///     use url::Url;
///
///     let now : DateTime<Utc> = Utc::now();
///     let url = Url::parse("https://example.com/launch").unwrap();
///
///     let req = AgsScoreUpdateReqBody::builder()
///         .submitted_at(now)
///         .submit_with_launch_url(url)
///         .grading_pending_manual()
///         .build();
///
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsScoreUpdateReqBody<'a> {
    pub activity_progress: AgsActivityProgress,
    /// Canvas specific extension for providing submission details.
    #[serde(rename = "https://canvas.instructure.com/lti/submission")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canvas_submission: Option<CanvasAgsSubmission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<&'a str>,
    pub grading_progress: AgsGradingProgress,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_given: Option<Decimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_maximum: Option<Decimal>,
    /// LTI Spec submission metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission: Option<AgsSubmission>,
    /// Time of submission.
    pub timestamp: DateTime<Utc>,
    /// The LTI identifier of the student whom submitted this assignment.
    pub user_id: &'a str,
}

#[derive(Default)]
pub struct AgsScoreUpdateBuilder<'a> {
    req: AgsScoreUpdateReqBody<'a>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsSubmission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission_url: Option<Url>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum AgsActivityProgress {
    Completed,
    #[default]
    Initialized,
    InProgress,
    Started,
    Submitted,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum AgsGradingProgress {
    Failed,
    FullyGraded,
    #[default]
    NotReady,
    Pending,
    PendingManual,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CanvasAgsOnlineUpload {
    ContentItems {
        content_items: Box<[CanvasAgsSubmissionContentItem]>,
    },
    SubmissionData {
        submission_data: Box<str>,
    },
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CanvasAgsSubmission {
    /// true if Canvas should create a new submission regardless of
    /// pre-existing submissions.
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub new_submission: bool,
    /// true if Canvas should ignore the score in this request.
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub preserve_score: bool,
    /// true if Canvas should prefer its own grade over one provided
    /// in this request.
    #[serde(default, skip_serializing_if = "<&bool>::not")]
    pub prioritize_non_tool_grade: bool,
    /// Type of submission, this informs Canvas as to where the
    /// submission data might be.
    #[serde(
        default,
        flatten,
        skip_serializing_if = "CanvasAgsSubmissionType::is_external_tool"
    )]
    pub submission_type: CanvasAgsSubmissionType,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CanvasAgsSubmissionContentItem {
    #[serde(default, rename = "type")]
    pub content_type: CanvasAgsContentItemType,
    pub url: Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "submission_type")]
pub enum CanvasAgsSubmissionType {
    None,
    BasicLtiLaunch(CanvasAgsUrlSubmission),
    OnlineTextEntry {
        submission_data: Box<str>,
    },
    #[default]
    ExternalTool,
    OnlineUpload(CanvasAgsOnlineUpload),
    OnlineUrl(CanvasAgsUrlSubmission),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum CanvasAgsContentItemType {
    #[default]
    #[serde(rename = "file")]
    File,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CanvasAgsUrlSubmission {
    #[serde(rename = "submission_data")]
    url: Url,
}

impl<'a> AgsScoreUpdateReqBody<'a> {
    pub fn builder() -> AgsScoreUpdateBuilder<'a> {
        AgsScoreUpdateBuilder {
            req: Self::default(),
        }
    }
}

impl CanvasAgsSubmissionType {
    pub fn is_external_tool(&self) -> bool {
        matches!(*self, Self::ExternalTool)
    }
}

impl<'a> AgsScoreUpdateBuilder<'a> {
    pub fn build(self) -> AgsScoreUpdateReqBody<'a> {
        self.req
    }

    pub fn comment(mut self, value: &'a str) -> Self {
        self.req.comment = Some(value);

        self
    }

    pub fn grading_pending_manual(mut self) -> Self {
        self.req.grading_progress = AgsGradingProgress::PendingManual;

        self
    }

    pub fn score(mut self, given: Decimal, max: Decimal) -> Self {
        self.req.score_given = Some(given);
        self.req.score_maximum = Some(max);

        self
    }

    /// Canvas will create a new submission instead of updating
    /// an existing one.
    pub fn new_submission(mut self) -> Self {
        let mut cs = self.req.canvas_submission.unwrap_or_default();

        cs.new_submission = true;

        self.req.canvas_submission = Some(cs);

        self
    }

    /// Canvas will not change the grade.
    pub fn preserve_score(mut self) -> Self {
        let mut cs = self.req.canvas_submission.unwrap_or_default();

        cs.preserve_score = true;

        self.req.canvas_submission = Some(cs);

        self
    }

    /// Canvas will prefer its own grade over one provided
    /// in this request.
    pub fn prioritize_non_tool_grade(mut self) -> Self {
        let mut cs = self.req.canvas_submission.unwrap_or_default();

        cs.prioritize_non_tool_grade = true;

        self.req.canvas_submission = Some(cs);

        self
    }

    pub fn submitted_at(mut self, value: DateTime<Utc>) -> Self {
        let mut sub = self.req.submission.unwrap_or_default();

        sub.submitted_at = Some(value);
        self.req.timestamp = value;
        self.req.submission = Some(sub);

        self
    }

    /// This submission is tied to a launch url which speedgrader
    /// will use to access the submission contents.
    pub fn submit_with_launch_url(mut self, url: Url) -> Self {
        let mut cs = self.req.canvas_submission.unwrap_or_default();

        let launch = CanvasAgsUrlSubmission { url };

        cs.submission_type = CanvasAgsSubmissionType::BasicLtiLaunch(launch);

        self.req.activity_progress = AgsActivityProgress::Submitted;
        self.req.canvas_submission = Some(cs);

        self
    }

    /// The LTI identifier of the student whom submitted this assignment.
    pub fn user_id(mut self, value: &'a str) -> Self {
        self.req.user_id = value;

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn it_should_serialize() {
        let now: DateTime<Utc> = Utc::now();
        let url = Url::parse("https://example.com/launch").unwrap();

        let req = AgsScoreUpdateReqBody::builder()
            .comment("test")
            .submitted_at(now)
            .submit_with_launch_url(url)
            .grading_pending_manual()
            .user_id("foo")
            .build();

        let body = serde_json::to_value(&req).unwrap();

        assert_eq!("Submitted", body["activityProgress"]);

        assert_eq!(
            "https://example.com/launch",
            body["https://canvas.instructure.com/lti/submission"]["submission_data"]
        );

        assert_eq!(
            "basic_lti_launch",
            body["https://canvas.instructure.com/lti/submission"]["submission_type"]
        );

        assert_eq!("test", body["comment"]);

        assert_eq!("PendingManual", body["gradingProgress"]);

        assert_eq!("foo", body["userId"])
    }
}

pub mod ags;
pub mod canvas;
pub mod domain;
pub mod error;
pub mod lti;
pub mod oidc;
pub mod util;

pub mod prelude {
    pub use super::lti::{
        LineItem, OidcConfiguration, RegistrationRequest, RegistrationResponse,
        SubmissionReviewRequest, SubmissionReviewRequestOptions, ToolConfiguration,
    };

    pub use super::ags::claims::ags_endpoint::AgsEndpoint;
    pub use super::ags::claims::ags_scope::AgsScope;
    pub use super::ags::line_items_request::AgsLineItemsReqParams;
    pub use super::ags::line_items_response::AgsLineItemsResponse;
    pub use super::ags::score_update_request::{
        AgsActivityProgress, AgsGradingProgress, AgsScoreUpdateReqBody, AgsSubmission,
    };
}

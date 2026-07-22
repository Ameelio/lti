#[cfg(feature = "axum")]
pub mod claims {
    pub use lti_claims::*;
}

#[cfg(feature = "axum")]
pub mod launch {
    pub use lti_launch::*;
}

pub mod spec {
    pub use lti_spec::*;
}

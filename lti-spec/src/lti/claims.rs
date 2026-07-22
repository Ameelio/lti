//! LTI specific claims
//! See [Lti Message Spec](https://www.imsglobal.org/spec/lti-dr/v1p0#lti-message)

pub mod canvas_privacy_level;
pub mod content_item;
pub mod context;
pub mod custom_params;
pub mod deep_linking_settings;
pub mod for_user;
pub mod launch_presentation;
pub mod platform_instance;
pub mod resource_link;
pub mod response_auth;
pub mod role;
pub mod version;

pub mod prelude {
    pub use super::content_item::{ContentItem, ContentItemType};
    pub use super::context::{Context, ContextType};
    pub use super::custom_params::CustomParams;
    pub use super::deep_linking_settings::{self, DeepLinkingSettings};
    pub use super::launch_presentation::{self, LaunchPresentation};
    pub use super::platform_instance::PlatformInstance;
    pub use super::resource_link::ResourceLink;
    pub use super::response_auth::ResponseAuth;
    pub use super::role::Role;
    pub use super::version::Version;
}

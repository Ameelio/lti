use serde::{Deserialize, Serialize};
use url::Url;

use crate::canvas::{CanvasDisplayType, CanvasLaunchMeasurement, CanvasVisibility};
use crate::lti::claims::custom_params::CustomParams;
use crate::lti::claims::role::Role;
use crate::lti::placement::Placement;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResourceLinkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<CustomParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://canvas.instructure.com/lti/display_type")]
    pub display_type: Option<CanvasDisplayType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://canvas.instructure.com/lti/launch_height")]
    pub launch_height: Option<CanvasLaunchMeasurement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://canvas.instructure.com/lti/launch_width")]
    pub launch_width: Option<CanvasLaunchMeasurement>,

    pub placements: Box<[Placement]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Box<[Role]>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_link_uri: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "https://canvas.instructure.com/lti/visibility")]
    pub visibility: Option<CanvasVisibility>,
}

impl ResourceLinkConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_custom_param(
        mut self,
        key: impl Into<Box<str>>,
        value: impl Into<Box<str>>,
    ) -> Self {
        let mut params = self.custom_parameters.unwrap_or_default();

        params.insert(key, value);

        self.custom_parameters = Some(params);
        self
    }

    pub fn add_placement(mut self, value: Placement) -> Self {
        let mut vec: Vec<Placement> = self.placements.into_vec();

        vec.push(value);

        self.placements = vec.into_boxed_slice();

        self
    }

    pub fn add_role(mut self, value: Role) -> Self {
        let slice: Box<[Role]> = self.roles.unwrap_or_default();

        let mut vec: Vec<Role> = slice.to_vec();

        vec.push(value);

        self.roles = Some(vec.into_boxed_slice());

        self
    }

    pub fn set_display_type(mut self, value: CanvasDisplayType) -> Self {
        self.display_type = Some(value);

        self
    }

    pub fn set_icon_uri(mut self, value: Url) -> Self {
        self.icon_uri = Some(value);

        self
    }

    pub fn set_label(mut self, value: impl Into<Box<str>>) -> Self {
        self.label = Some(value.into());

        self
    }

    pub fn set_launch_height(mut self, value: CanvasLaunchMeasurement) -> Self {
        self.launch_height = Some(value);

        self
    }

    pub fn set_launch_width(mut self, value: CanvasLaunchMeasurement) -> Self {
        self.launch_width = Some(value);

        self
    }

    pub fn set_target_link_uri(mut self, value: Url) -> Self {
        self.target_link_uri = Some(value);

        self
    }

    pub fn set_visibility(mut self, value: CanvasVisibility) -> Self {
        self.visibility = Some(value);

        self
    }
}

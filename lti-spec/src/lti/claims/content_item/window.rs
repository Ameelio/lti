use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Window {
    pub height: u64,
    /// comma seperated list of window features.
    #[serde(default, rename = "windowFeatures")]
    pub features: WindowFeatures,
    /// target window, this allows for all content items to reuse in a single window.
    #[serde(rename = "targetName")]
    pub target_name: Box<str>,
    pub width: u64,
}

/// Window features as passed to window.open()
/// See: [window.open() definition](https://developer.mozilla.org/en-US/docs/Web/API/Window/open).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowFeatures {
    /// height in pixels including scrollbars
    pub inner_height: Option<u16>,
    /// width in pixels including scrollbars
    pub inner_width: Option<u16>,
    pub outlet: WindowOutlet,
    /// vertical pixels to offset from the user position
    pub screen_x: Option<i16>,
    /// vertical pixels to offset from the user position
    pub screen_y: Option<i16>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum WindowOutlet {
    Popup,
    #[default]
    Tab,
}

impl Default for WindowFeatures {
    fn default() -> Self {
        Self {
            inner_height: None,
            inner_width: None,
            outlet: WindowOutlet::default(),
            screen_x: None,
            screen_y: None,
        }
    }
}

impl<'de> Deserialize<'de> for WindowFeatures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut value = Self::default();

        let input: &str = Deserialize::deserialize(deserializer)?;

        for pair in input.split(",") {
            let Some((field_name, field_value)) = pair.split_once('=') else {
                continue;
            };

            match field_name {
                "inner_height" => {
                    let field_value: u16 = field_value.parse().map_err(serde::de::Error::custom)?;
                    value.inner_height = Some(field_value);
                }
                "inner_width" => {
                    let field_value: u16 = field_value.parse().map_err(serde::de::Error::custom)?;
                    value.inner_width = Some(field_value);
                }
                "popup" => {
                    value.outlet = WindowOutlet::Popup;
                }
                "screen_x" => {
                    let field_value: i16 = field_value.parse().map_err(serde::de::Error::custom)?;
                    value.screen_x = Some(field_value);
                }
                "screen_y" => {
                    let field_value: i16 = field_value.parse().map_err(serde::de::Error::custom)?;
                    value.screen_y = Some(field_value);
                }
                _ => {}
            };
        }

        Ok(value)
    }
}
impl Serialize for WindowFeatures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tokens: Vec<String> = Vec::with_capacity(5);

        if let Some(inner_height) = self.inner_height {
            let token = format!("inner_height={}", inner_height);
            tokens.push(token);
        };

        if let Some(inner_width) = self.inner_width {
            let token = format!("inner_width={}", inner_width);
            tokens.push(token);
        };

        match self.outlet {
            WindowOutlet::Popup => {
                let token = format!("popup=true");
                tokens.push(token)
            }
            _ => {}
        };

        if let Some(screen_x) = self.screen_x {
            let token = format!("screen_x={}", screen_x);
            tokens.push(token);
        };

        if let Some(screen_y) = self.screen_y {
            let token = format!("screen_y={}", screen_y);
            tokens.push(token);
        };

        let value: String = tokens.join(",");

        serializer.serialize_str(value.as_str())
    }
}

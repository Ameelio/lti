use serde::{Deserialize, Serialize};
use std::fmt;

/// This represents situations where if a value is not present
/// it should not be treated as false but unknown.
#[derive(Clone, Debug, Default)]
pub enum Toggle {
    On,
    Off,
    #[default]
    Unknown,
}

impl Toggle {
    pub fn is_off(&self) -> bool {
        match self {
            Self::Off => true,
            _ => false,
        }
    }

    pub fn is_on(&self) -> bool {
        match self {
            Self::On => true,
            _ => false,
        }
    }

    pub fn is_unkown(&self) -> bool {
        match self {
            Self::Unknown => true,
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for Toggle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor)
    }
}

impl Serialize for Toggle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Toggle::On => serializer.serialize_some(&true),
            Toggle::Off => serializer.serialize_some(&false),
            Toggle::Unknown => serializer.serialize_none(),
        }
    }
}

impl PartialEq for Toggle {
    /// Match if On or Off, but OptionToggle::Unknown is not equal to anything.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::On, Self::On) | (Self::Off, Self::Off) => true,
            _ => false,
        }
    }
}

impl Eq for Toggle {}

struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Toggle;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("expecting true, false, or null")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let v: Toggle = match v {
            true => Toggle::On,
            false => Toggle::Off,
        };

        Ok(v)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: bool = Deserialize::deserialize(deserializer)?;

        self.visit_bool(value)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Toggle::Unknown)
    }
}

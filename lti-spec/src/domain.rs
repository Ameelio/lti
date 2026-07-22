//! handles the LTI HTTP API
//!
use std::fmt;

use serde::{de, ser, Deserialize, Serialize};
use url::{Host, Url};

use crate::error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Domain(pub Host);

impl<'de> Deserialize<'de> for Domain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Domain;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value: Host = Host::parse(v).map_err(de::Error::custom)?;

                Ok(Domain(value))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl Default for Domain {
    fn default() -> Self {
        let host = String::from("localhost");
        let host = Host::Domain(host);
        Self(host)
    }
}

impl Serialize for Domain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let value: String = self.0.to_string();

        serializer.serialize_str(value.as_str())
    }
}

impl TryFrom<Url> for Domain {
    type Error = Error;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        let Some(value) = value.domain() else {
            return Err(Error::InvalidDomain);
        };

        let value = Host::parse(value).map_err(|source| Error::DomainParseError { source })?;

        Ok(Self(value))
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn it_should_serialize_and_deserialize() {
        let input = json!({
            "domain": "foo-staging.example.com"
        });

        #[derive(Deserialize, Serialize)]
        struct HasDomain {
            domain: Domain,
        }

        let hd: HasDomain = serde_json::from_value(input).unwrap();

        let hd: String = serde_json::to_string(&hd).unwrap();

        let expected = r#"{"domain":"foo-staging.example.com"}"#;

        assert_eq!(expected, hd.as_str())
    }
}

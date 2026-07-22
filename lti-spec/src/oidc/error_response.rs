use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ErrorResponse {
    pub code: Box<str>,
    pub description: Box<str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Box<[Box<str>]>>,
}

impl ErrorResponse {
    pub fn internal<T>(description: T, error: T) -> Self
    where
        T: Into<Box<str>>,
    {
        let errors: Box<[Box<str>]> = Box::new([error.into()]);
        Self {
            code: "internal_sever_error".into(),
            description: description.into(),
            errors: Some(errors),
        }
    }

    pub fn not_found<T>(description: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self {
            code: "not_found".into(),
            description: description.into(),
            errors: Option::None,
        }
    }

    pub fn unauthorized<T>(description: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self {
            code: "unauthorized".into(),
            description: description.into(),
            errors: Option::None,
        }
    }

    pub fn unavailable<T>(description: T) -> Self
    where
        T: Into<Box<str>>,
    {
        Self {
            code: "unavailable".into(),
            description: description.into(),
            errors: Option::None,
        }
    }
}

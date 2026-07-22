use std::collections::HashSet;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UrlSet(HashSet<Url>);

impl UrlSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, value: &url::Url) -> bool {
        let host: Box<str> = value.host_str().unwrap_or_default().into();
        let path: Box<str> = value.path().into();
        let value = Url { host, path };

        self.0.get(&value).is_some()
    }
    pub fn insert(&mut self, value: url::Url) -> bool {
        let host: Box<str> = value.host_str().unwrap_or_default().into();
        let path: Box<str> = value.path().into();

        let value = Url { host, path };
        self.0.insert(value)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Url {
    host: Box<str>,
    path: Box<str>,
}

use serde::Deserialize;
use zeroize::Zeroizing;

#[derive(Deserialize)]
pub struct ResponseBody {
    pub access_token: Zeroizing<Box<str>>,
}

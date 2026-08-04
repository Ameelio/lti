use std::time::Duration;

use json_web_key::jwk::JsonWebKeySet;
use json_web_tolkien::jws::Jws;
use tracing::instrument;

use crate::ClaimsState;
use crate::error::Error;
use crate::lms_client::LmsClient;

#[instrument(err, skip(signature))]
pub async fn verify_signature<S>(
    state: &S,
    lms_client: &LmsClient,
    signature: Jws,
) -> Result<(), Error>
where
    S: ClaimsState,
{
    let key_id: &str = &lms_client.key_id;

    let keys: JsonWebKeySet = {
        let jwks_url: &str = lms_client.jwks_url.as_str();
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_millis(500))
            .build()?;
        let keys: String = http_client.get(jwks_url).send().await?.text().await?;

        serde_json::from_str(&keys)?
    };

    state
        .validate_signature(key_id, keys, signature)
        .map_err(|source| Error::InvalidSignature { source })?;

    Ok(())
}

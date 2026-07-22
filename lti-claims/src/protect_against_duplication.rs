use serde::Deserialize;

use tracing::instrument;

use crate::authenticatable::Authenticatable;
use crate::claims::IdToken;
use crate::claims_state::{ClaimsState, LockResult};
use crate::error::Error;

/// This looks at a nonce provided within the jwt to determine if the request is a duplicate.
#[instrument(err, skip(id_token))]
pub async fn protect_against_duplication<A, S>(
    state: &S,
    id_token: &IdToken<A>,
) -> Result<(), Error>
where
    A: Authenticatable + for<'de> Deserialize<'de>,
    S: ClaimsState,
{
    let nonce: &str = id_token.nonce();

    match state.try_lock(nonce).await {
        LockResult::Ok => Ok(()),
        LockResult::DuplicateRequest => Err(Error::DuplicateRequest),
        LockResult::Err(source) => Err(Error::UnexpectedCacheError { source }),
    }
}

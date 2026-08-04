use chrono::Utc;
use serde::Deserialize;
use tracing::instrument;

use crate::authenticatable::Authenticatable;
use crate::error::Error;
use crate::lms_client::LmsClient;

#[instrument(err, skip(auth))]
pub fn validate_claims<A>(auth: &A, lms_client: &LmsClient) -> Result<(), Error>
where
    A: Authenticatable + for<'de> Deserialize<'de>,
{
    let now = Utc::now();

    if let Some(azp) = auth.authorized_party()
        && azp != lms_client.client_id.as_ref()
    {
        return Err(Error::ClientNotAnAzp);
    }

    if !auth
        .audience()
        .iter()
        .any(|x: &str| x == lms_client.client_id.as_ref())
    {
        return Err(Error::ClientNotInAud);
    }

    if auth.expired_at() < now {
        return Err(Error::ExpiredToken {
            expired_at: auth.expired_at().to_rfc3339().into(),
        });
    }

    if auth.issued_at() > now {
        return Err(Error::IssuedAtInFuture {
            issued_at: auth.issued_at().to_rfc3339().into(),
        });
    }

    Ok(())
}

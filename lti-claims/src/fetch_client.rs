use serde::Deserialize;
use serde_jwt::util::OneOrMore;
use tracing::instrument;
use url::Url;

use crate::claims::Header;
use crate::claims::IdToken;
use crate::error::Error;
use crate::lms_client::LmsClient;
use crate::{Authenticatable, ClaimsState};

#[instrument(err, skip(id_token))]
pub fn fetch_client<A, C>(state: &C, id_token: &IdToken<A>) -> Result<LmsClient, Error>
where
    A: Authenticatable + for<'de> Deserialize<'de>,
    C: ClaimsState,
{
    let key_id: &str = {
        let header: &Header = id_token.header();
        header.kid.as_ref()
    };

    let client_ids: Vec<&str> = match id_token.authorized_party() {
        Some(azp) => vec![azp],
        None => match id_token.audience() {
            OneOrMore::One(aud) => vec![aud.as_ref()],
            OneOrMore::More(aud) => aud.as_ref().iter().map(|x| x.as_ref()).collect(),
        },
    };

    // We need to convert to url to not only validate the
    // issuer but also, ensure it matches properly with the
    // database record.
    let issuer =
        Url::parse(id_token.issuer()).map_err(|source| Error::InvalidIssuerUrl { source })?;

    let lms_client: LmsClient = state.fetch_client(client_ids, issuer.as_str(), key_id)?;

    Ok(lms_client)
}

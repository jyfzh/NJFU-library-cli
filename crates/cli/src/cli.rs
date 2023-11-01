use super::action::Action::{self, *};
use super::reserve::Reserve;
use anyhow::{anyhow, Result};
use core::error::ClientError;
use core::njfulib::resp::Resp;

pub async fn handle_action(action: Action) -> Result<Resp> {
    // Perform the action.
    match action {
        Login {
            username,
            password,
            cookie,
        } => core::login(username, password, cookie),

        Status { day } => core::status(day).await,

        Query {
            day,
            name,
            site,
            filter,
        } => {
            if let Some(name) = name {
                core::query_by_name(day, name, filter).await
            } else if let Some(site) = site {
                core::query_by_site(day, site).await
            } else {
                Err(anyhow!(ClientError::InputError(
                    "no query name or site".to_string()
                )))
            }
        }

        Cancel { uuid } => core::cancel(uuid).await,

        Reserve(Reserve {
            sites,
            filter,
            day,
            start,
            end,
            retry,
        }) => core::reserve(sites, filter, day, start, end, retry).await,
    }
}

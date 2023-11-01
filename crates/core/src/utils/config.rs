use crate::def;
use crate::error::ClientError;
use crate::njfulib::resp::{Data, Resp};
use crate::njfulib::user::*;
use anyhow::anyhow;
use anyhow::Result;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

/// Info struct is used to store the information of the user's state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub cookie: String,
    pub user: Option<User>,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", def::LONG_LINE_SEPARATOR)?;
        writeln!(f, "username: {}", self.username)?;
        writeln!(f, "password: {}", self.password)?;
        writeln!(f, "cookie: {}", self.cookie)?;
        if let Some(user) = &self.user {
            write!(f, "{}", user)?;
        }
        Ok(())
    }
}

impl Config {
    pub fn new(username: String, password: String, cookie: String, user: Option<User>) -> Self {
        Config {
            username,
            password,
            cookie,
            user,
        }
    }
}

pub fn save_config_to_file(config: &Config) -> Result<()> {
    let root = home_dir().ok_or(ClientError::Config)?;
    let path = root.join(def::CONFIG_FILE);

    let mut output = File::create(path)?;
    let info = serde_json::to_string_pretty(&config)?;
    write!(output, "{}", info)?;
    Ok(())
}

pub fn load_config_from_file() -> Result<Config> {
    let root = home_dir().ok_or(ClientError::Config)?;
    let path = root.join(def::CONFIG_FILE);

    let input = File::open(path)?;
    let config: Config = serde_json::from_reader(input)?;

    if config.username.is_empty() || config.password.is_empty() || config.cookie.is_empty() {
        return Err(anyhow!(ClientError::Config));
    }
    Ok(config)
}

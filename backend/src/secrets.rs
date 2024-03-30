use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::process::exit;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use toml::from_str;
use tracing::{error, instrument};

use crate::config::{try_parse_config, MycologConfig};

#[instrument]
pub fn parse_secrets() -> MycologSecrets {
    match try_parse_secrets() {
        Ok(secrets) => secrets,
        Err(err) => {
            error!(err = err.to_string(), "secrets unable to read due to error");
            exit(3);
        }
    }
}

pub fn try_parse_secrets() -> anyhow::Result<MycologSecrets> {
    let keys_file = try_read_secrets_keys()?;
    let db_file = try_read_secrets_db()?;

    let mailersend = keys_file.mailersend.ok_or(anyhow!(
        "key for `mailersend` in secrets/keys.toml is missing"
    ))?;

    let db_user = db_file
        .user
        .ok_or(anyhow!("value for `user` in secrets/db.toml is missing"))?;
    let db_password = db_file.password.ok_or(anyhow!(
        "password for `password` in secrets/db.toml is missing"
    ))?;

    Ok(MycologSecrets {
        keys: SecretsKeys { mailersend },
        db: SecretsDb {
            user: db_user,
            password: db_password,
        },
    })
}

fn try_read_secrets_keys() -> anyhow::Result<SecretsKeysFile> {
    let mut keys_file = File::open("secrets/keys.toml")?;
    let mut read_keys_file = String::new();
    keys_file.read_to_string(&mut read_keys_file)?;
    Ok(from_str(&read_keys_file)?)
}

fn try_read_secrets_db() -> anyhow::Result<SecretsDbFile> {
    let mut db_file = File::open("secrets/db.toml")?;
    let mut read_db_file = String::new();
    db_file.read_to_string(&mut read_db_file)?;
    Ok(from_str(&read_db_file)?)
}

#[derive(Clone, Debug)]
pub struct MycologSecrets {
    pub keys: SecretsKeys,
    pub db: SecretsDb,
}

#[derive(Clone)]
pub struct SecretsKeys {
    mailersend: String,
}

impl SecretsKeys {
    pub fn mailersend(&self) -> String {
        self.mailersend.clone()
    }
}

impl Debug for SecretsKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretsKeys")
            .field("mailersend", &"?")
            .finish()
    }
}

#[derive(Clone)]
pub struct SecretsDb {
    user: String,
    password: String,
}

impl SecretsDb {
    pub fn user(&self) -> String {
        self.user.clone()
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }
}

impl Debug for SecretsDb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretsDB")
            .field("user", &self.user)
            .field("password", &"?")
            .finish()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SecretsKeysFile {
    mailersend: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SecretsDbFile {
    user: Option<String>,
    password: Option<String>,
}

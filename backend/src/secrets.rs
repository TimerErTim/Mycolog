use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::process::exit;

use anyhow::{anyhow, bail};
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
    let mut keys_file = try_read_secrets_keys()?;
    let db_file = try_read_secrets_db()?;
    let admin_file = try_read_secrets_admin()?;

    let Some(mailersend_file) = keys_file.mailersend else {
        bail!("no section for `mailersend` in secrets/keys.toml");
    };
    let mailersend_api = mailersend_file.api_key.ok_or(anyhow!(
        "value for `mailersend.api_key` in secrets/keys.toml is missing"
    ))?;
    let mailersend_webhook = mailersend_file.webhook_signature.ok_or(anyhow!(
        "value for `mailersend.webhook_signature` in secrets/keys.toml is missing"
    ))?;

    let db_user = db_file
        .user
        .ok_or(anyhow!("value for `user` in secrets/db.toml is missing"))?;
    let db_password = db_file.password.ok_or(anyhow!(
        "password for `password` in secrets/db.toml is missing"
    ))?;

    let admin_token = admin_file
        .token
        .ok_or(anyhow!(
            "value for `token` in secrets/admin.toml is missing"
        ))?
        .chars()
        .filter(|c| c.is_ascii())
        .collect::<String>();

    Ok(MycologSecrets {
        keys: SecretsKeys {
            mailersend_api,
            mailersend_webhook,
        },
        db: SecretsDb {
            user: db_user,
            password: db_password,
        },
        admin: SecretsAdmin { token: admin_token },
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

fn try_read_secrets_admin() -> anyhow::Result<SecretsAdminFile> {
    let mut admin_file = File::open("secrets/admin.toml")?;
    let mut read_admin_file = String::new();
    admin_file.read_to_string(&mut read_admin_file)?;
    Ok(from_str(&read_admin_file)?)
}

#[derive(Clone, Debug)]
pub struct MycologSecrets {
    pub keys: SecretsKeys,
    pub db: SecretsDb,
    pub admin: SecretsAdmin,
}

#[derive(Clone)]
pub struct SecretsKeys {
    mailersend_api: String,
    mailersend_webhook: String,
}

impl SecretsKeys {
    pub fn mailersend_api(&self) -> String {
        self.mailersend_api.clone()
    }

    pub fn mailersend_webhook(&self) -> String {
        self.mailersend_webhook.clone()
    }
}

impl Debug for SecretsKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretsKeys")
            .field("mailersend_api", &"?")
            .field("mailersend_webhook", &"?")
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

#[derive(Clone)]
pub struct SecretsAdmin {
    token: String,
}

impl SecretsAdmin {
    pub fn token(&self) -> String {
        self.token.clone()
    }
}

impl Debug for SecretsAdmin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecretsAdmin").field("token", &"?").finish()
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SecretsKeysFile {
    mailersend: Option<KeysFileMailersend>,
}

#[derive(Clone, Serialize, Deserialize)]
struct KeysFileMailersend {
    api_key: Option<String>,
    webhook_signature: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SecretsDbFile {
    user: Option<String>,
    password: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SecretsAdminFile {
    token: Option<String>,
}

use std::fmt::Arguments;
use std::fs::{write, File};
use std::io::Read;
use std::net::IpAddr;
use std::process::exit;
use std::str::FromStr;

use anyhow::bail;
use email_address_parser::EmailAddress;
use serde::{Deserialize, Serialize};
use toml::from_str;
use tracing::{error, instrument, warn};

use crate::application::BackupLimit;
use crate::cli::MycologArguments;

pub fn parse_config(arguments: MycologArguments) -> MycologConfig {
    match try_parse_config(arguments) {
        Ok(config) => config,
        Err(err) => {
            error!(%err, "configuration failed to load due to error");
            exit(2);
        }
    }
}

#[instrument]
pub fn try_parse_config(arguments: MycologArguments) -> anyhow::Result<MycologConfig> {
    let config_file = try_read_config()?;
    let default_config = MycologConfig::default();

    let mut should_write_config = false;

    let web_file = match &config_file.web {
        Some(file) => file.clone(),
        None => Default::default(),
    };
    let web_bind_ip = if let Some(web_bind_ip) = &web_file.ip {
        IpAddr::from_str(web_bind_ip)?
    } else {
        warn!("`web.ip` is missing from config");
        should_write_config = true;
        default_config.web_bind_ip
    };
    let web_bind_port = if let Some(web_bind_port) = &web_file.port {
        *web_bind_port
    } else {
        warn!("`web.port` is missing from config");
        should_write_config = true;
        default_config.web_bind_port
    };

    let email_file = match &config_file.email {
        Some(file) => file.clone(),
        None => Default::default(),
    };
    let email_noreply_sender = if let Some(email_noreply_sender) = &email_file.noreply_sender
        && email_noreply_sender.is_empty()
    {
        if EmailAddress::is_valid(email_noreply_sender, None) {
            bail!("invalid `email.noreply_sender` in config");
        }
        email_noreply_sender.clone()
    } else {
        warn!("`email.noreply_sender` is is missing from config");
        should_write_config = true;
        default_config.email_noreply_sender
    };

    let images_file = match &config_file.images {
        Some(file) => file.clone(),
        None => Default::default(),
    };
    let images_max_bytes_per_user =
        if let Some(images_max_bytes_per_user) = images_file.max_bytes_per_user {
            if images_max_bytes_per_user <= 0 {
                bail!("`images.max_bytes_per_user` must be greater than 0");
            }
            images_max_bytes_per_user
        } else {
            warn!("`images.max_bytes_per_user` is is missing from config");
            should_write_config = true;
            default_config.images_max_bytes_per_user
        };

    let backup_file = match &config_file.backups {
        Some(file) => file.clone(),
        None => Default::default(),
    };
    let backup_delay_hours = if let Some(backup_delay_hours) = &backup_file.delay_hours {
        *backup_delay_hours
    } else {
        warn!("`backups.delay_hours` is is missing from config");
        should_write_config = true;
        default_config.backup_delay_hours
    };

    let backup_interval_hours = if let Some(backup_interval_hours) = &backup_file.interval_hours {
        *backup_interval_hours
    } else {
        warn!("`backups.interval_hours` is is missing from config");
        should_write_config = true;
        default_config.backup_interval_hours
    };

    let mut found_backup_limit = None;
    if let Some(backup_max_amount) = &backup_file.max_amount {
        found_backup_limit = Some(BackupLimit::MaxAmountBackups(*backup_max_amount));
    };
    if let Some(backup_max_size) = &backup_file.max_size {
        if found_backup_limit.is_some() {
            bail!("only one of `backups.max_age`, `backups.max_size` or `backups.max_amount` may be specified");
        }
        found_backup_limit = Some(BackupLimit::MaxSumSizeMB(*backup_max_size));
    };
    if let Some(backups_max_age) = &backup_file.max_age {
        if found_backup_limit.is_some() {
            bail!("only one of `backups.max_age`, `backups.max_size` or `backups.max_amount` may be specified");
        }
        found_backup_limit = Some(BackupLimit::MaxAgeHours(*backups_max_age));
    };
    let Some(backup_limit) = found_backup_limit else {
        bail!("neither `backups.max_age`, `backups.max_size` nor `backups.max_amount` was found in config");
    };

    let mut config = MycologConfig {
        web_bind_ip,
        web_bind_port,
        email_noreply_sender,
        images_max_bytes_per_user,
        backup_delay_hours,
        backup_interval_hours,
        backup_limit,
    };

    if should_write_config {
        if let Err(err) = try_write_config(&config) {
            warn!(%err, "config parsing failed due to locked file");
        }
    }

    if let Some(port) = arguments.port {
        config.web_bind_port = port;
    }
    if let Some(ip) = arguments.hostname {
        config.web_bind_ip = ip;
    }

    Ok(config)
}

fn try_read_config() -> anyhow::Result<ConfigFile> {
    let mut config_file = File::open("config/config.toml")?;
    let mut read_config_file = String::new();
    config_file.read_to_string(&mut read_config_file)?;
    Ok(from_str(&read_config_file)?)
}

fn try_write_config(config: &MycologConfig) -> anyhow::Result<()> {
    let parsed_config_file: ConfigFile = config.into();
    Ok(write(
        "config/config.toml",
        toml::to_string(&parsed_config_file)?,
    )?)
}

impl Default for MycologConfig {
    fn default() -> Self {
        Self {
            web_bind_ip: IpAddr::from([127, 0, 0, 1]),
            web_bind_port: 8031,
            email_noreply_sender: "noreply@example.com".to_string(),
            images_max_bytes_per_user: 2u64.pow(30), // 1GB,
            backup_delay_hours: 24,
            backup_interval_hours: 24,
            backup_limit: BackupLimit::MaxAmountBackups(7),
        }
    }
}

impl From<MycologConfig> for ConfigFile {
    fn from(value: MycologConfig) -> Self {
        (&value).into()
    }
}

impl From<&MycologConfig> for ConfigFile {
    fn from(value: &MycologConfig) -> Self {
        let backup_conf = match value.backup_limit.clone() {
            BackupLimit::MaxAmountBackups(max) => BackupConfig {
                max_amount: Some(max),
                ..Default::default()
            },
            BackupLimit::MaxSumSizeMB(max) => BackupConfig {
                max_size: Some(max),
                ..Default::default()
            },
            BackupLimit::MaxAgeHours(max) => BackupConfig {
                max_age: Some(max),
                ..Default::default()
            },
        };

        ConfigFile {
            email: Some(EmailConfig {
                noreply_sender: Some(value.email_noreply_sender.clone()),
            }),
            images: Some(ImagesConfig {
                max_bytes_per_user: Some(value.images_max_bytes_per_user),
            }),
            web: Some(WebConfig {
                ip: Some(value.web_bind_ip.to_string()),
                port: Some(value.web_bind_port),
            }),
            backups: Some(BackupConfig {
                delay_hours: Some(value.backup_delay_hours),
                interval_hours: Some(value.backup_interval_hours),
                ..backup_conf
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MycologConfig {
    // Web
    pub web_bind_ip: IpAddr,
    pub web_bind_port: u16,

    // Email
    pub email_noreply_sender: String,

    // Images
    pub images_max_bytes_per_user: u64,

    // Backups
    pub backup_delay_hours: u64,
    pub backup_interval_hours: u64,
    pub backup_limit: BackupLimit,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct ConfigFile {
    email: Option<EmailConfig>,
    images: Option<ImagesConfig>,
    web: Option<WebConfig>,
    backups: Option<BackupConfig>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct EmailConfig {
    noreply_sender: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct ImagesConfig {
    max_bytes_per_user: Option<u64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct WebConfig {
    ip: Option<String>,
    port: Option<u16>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct BackupConfig {
    delay_hours: Option<u64>,
    interval_hours: Option<u64>,
    max_amount: Option<u64>,
    max_size: Option<u64>,
    max_age: Option<u64>,
}

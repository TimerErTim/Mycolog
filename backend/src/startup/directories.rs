use std::fs::create_dir_all;
use std::path::Path;

use tracing::{info_span, warn};

pub fn prepare_application_dirs() -> anyhow::Result<()> {
    create_application_dirs()?;
    check_application_dirs()?;

    Ok(())
}

fn create_application_dirs() -> anyhow::Result<()> {
    create_dir_all("data/")?;
    create_dir_all("images/")?;
    create_dir_all("backups/")?;
    Ok(())
}

fn check_application_dirs() -> anyhow::Result<()> {
    check_dir_all("log/")?;
    check_dir_all("site/")?;
    check_dir_all("secrets/")?;
    Ok(())
}

fn check_dir_all(path: impl Into<Path>) -> anyhow::Result<bool> {
    let path = path.into();
    let _span = info_span!("checking_directory", directory = path).entered();

    if !path.is_dir() {
        warn!(
            dir = path,
            "Directory is missing but required for application"
        );
        return Ok(false);
    }

    Ok(true)
}

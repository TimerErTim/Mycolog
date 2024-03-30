use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum BackupLimit {
    // There may not be more backups than the specified amount
    MaxAmountBackups(u64),
    // The backup directory may not grow larger than the specified amount in megabytes
    MaxSumSizeMB(u64),
    // Oldest backup may not be older than the specified amount in hours
    MaxAgeHours(u64),
}

impl BackupLimit {
    pub fn check_exceeded(&self, paths: &[PathBuf]) -> bool {
        match self {
            BackupLimit::MaxAmountBackups(amount) => max_amount_backups(amount.clone(), paths),
            BackupLimit::MaxSumSizeMB(mb) => max_sum_size(mb.clone(), paths),
            BackupLimit::MaxAgeHours(hours) => max_age(hours.clone(), paths),
        }
    }
}

fn max_amount_backups(amount: u64, paths: &[PathBuf]) -> bool {
    paths.len() as u64 > amount
}

fn max_sum_size(mb: u64, paths: &[PathBuf]) -> bool {
    let mut sum_size = 0;
    for path in paths {
        if let Ok(metadata) = std::fs::metadata(path) {
            sum_size += metadata.size();
        }
    }

    sum_size / 2u64.pow(20) > mb
}

fn max_age(hours: u64, paths: &[PathBuf]) -> bool {
    for path in paths {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(created) = metadata.created() {
                if let Ok(elapsed) = created.elapsed() {
                    return elapsed.as_secs() / 3600 > hours;
                }
                break;
            }
        }
    }
    false
}

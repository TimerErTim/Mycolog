pub struct MycologConfig {
    // Backups
    pub backup_delay_hours: u64,
    pub backup_frequency_hours: u64,
    pub backup_max_amount: usize,
}

pub const GLOBAL_CONFIG: MycologConfig = MycologConfig {
    backup_frequency_hours: 48,
    backup_delay_hours: 0,
    backup_max_amount: 15,
};

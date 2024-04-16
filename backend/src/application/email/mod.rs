pub use manager::EmailManager;
pub use recipients::Recipient;

use crate::application::email::files::load_email_files;
use crate::application::DatabaseSystem;
use crate::config::MycologConfig;
use crate::secrets::MycologSecrets;

pub mod events;
mod files;
mod manager;
mod recipients;

pub async fn create_email_manager(
    config: &MycologConfig,
    secrets: &MycologSecrets,
    db: &DatabaseSystem,
) -> anyhow::Result<EmailManager> {
    let emails = load_email_files("emails/").await?;
    Ok(EmailManager::new(
        secrets,
        db.auth_root(),
        config.email_noreply_sender.clone(),
        emails,
    ))
}

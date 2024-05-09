use tracing::info;

pub use manager::ImageManager;
pub use manager::StoreImageError;
pub use manager::{read, write};

use crate::application::DatabaseSystem;
use crate::config::MycologConfig;
use crate::secrets::MycologSecrets;

mod data;
mod manager;

pub async fn create_image_manager(
    config: &MycologConfig,
    secrets: &MycologSecrets,
    db: &DatabaseSystem,
) -> anyhow::Result<ImageManager> {
    let db = db.auth_root();
    let manager = ImageManager::new("images/", db, config.images_max_bytes_per_user)?;
    info!("cleaning image manager during creation");
    manager.clean().await?;
    manager.constrain_images().await?;
    Ok(manager)
}

use std::collections::BTreeSet;

use anyhow::anyhow;
use serde::Deserialize;
use serde::__private::de::IdentifierDeserializer;
use surrealdb_core::sql;
use tracing::{debug, error, info, instrument, warn};

use crate::application::images::data::{ImageCleanInfo, ImageWriteInfo};
use crate::application::images::StoreImageError;
use crate::application::ImageManager;
use crate::utils::types::AnyhowExt;

impl ImageManager {
    #[instrument(skip_all, fields(self.folder = ? self.folder))]
    pub async fn clean(&self) -> anyhow::Result<()> {
        let db_images: Vec<ImageCleanInfo> = self
            .db
            .query("SELECT id, path FROM image;")
            .await?
            .take(0)?;
        let mut db_images = BTreeSet::from_iter(db_images);
        // Collect fs images
        let mut fs_images = BTreeSet::new();
        let mut read_dir = tokio::fs::read_dir(&self.folder).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let file_name = entry.file_name();
            let Some(file_name) = file_name.to_str() else {
                warn!(file = ?entry.path().display(), "image file unable to read filename");
                continue;
            };
            let file_name = file_name.to_string();
            fs_images.insert(file_name);
        }

        // Ignore already clean data
        for fs_image in fs_images.clone() {
            if db_images.contains(&fs_image) {
                db_images.remove(&fs_image);
                fs_images.remove(&fs_image);
            }
        }

        if !db_images.is_empty() {
            info!("cleaning {} images from database", db_images.len());

            let mut query = self.db.query(());
            for (index, db_image) in db_images.into_iter().enumerate() {
                query = query
                    .query(format!("DELETE $image_{index};"))
                    .bind(format!("image_{index}"), db_image.id);
            }
            if let Err(err) = query.await?.checked() {
                error!(?err, "failed cleaning images from database");
            }
        }

        if !fs_images.is_empty() {
            info!("cleaning {} images from filesystem", fs_images.len());

            for fs_image in fs_images {
                self.delete_image_by_path(fs_image).await;
            }
        }

        Ok(())
    }

    pub async fn constrain_images(&self) -> anyhow::Result<u32> {
        #[derive(Deserialize)]
        struct TotalBytesUsers {
            total_bytes: u64,
            owner: sql::Thing,
        }

        let mut amount_deleted = 0;
        let users = self.db.query("SELECT * FROM ( SELECT math::sum(file_size) AS total_bytes, owner FROM image GROUP BY owner ) WHERE total_bytes > $max_size;")
            .bind("max_size", self.max_bytes_per_user)
            .await?
            .take::<Vec<TotalBytesUsers>>(0)?;

        for mut user in users {
            while user.total_bytes > self.max_bytes_per_user {
                let delete_result = self.delete_oldest_image_by_owner(&user.owner).await
                    .inspect_err(|err| error!(%err, owner = user.owner.to_raw(), "error while deleting oldest image"));
                if let Ok(Some(image)) = delete_result {
                    debug!(owner = user.owner.to_raw(), image = ?image, "deleted image");
                    user.total_bytes -= image.file_size;
                    amount_deleted += 1;
                } else {
                    break;
                }
            }
        }

        Ok(amount_deleted)
    }

    pub async fn check_image(&self, info: &ImageWriteInfo) -> Result<(), StoreImageError> {
        let stored_bytes = self
            .db
            .query("math::sum( (SELECT VALUE file_size FROM image WHERE owner = $owner) );")
            .bind("owner", &info.owner)
            .await
            .anyhow()?
            .take::<Option<u64>>(0)
            .anyhow()?
            .ok_or(anyhow!("unable to get necessary storage space for user"))?;

        if stored_bytes + info.file_size > self.max_bytes_per_user {
            return Err(StoreImageError::StorageExceeded {
                amount: stored_bytes + info.file_size,
                limit: self.max_bytes_per_user,
            });
        }

        Ok(())
    }
}

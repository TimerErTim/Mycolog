use std::borrow::Borrow;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Cursor, ErrorKind};
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Error};
use axum::body::Bytes;
use image::{GenericImageView, ImageFormat};
use surrealdb_core::sql;
use thiserror::Error;
use tracing::{debug, error, info, instrument, Level};
use uuid::Uuid;

use crate::application::database::system::DatabaseScopeAccess;
use crate::application::images::data::{Dimensions, ImageId, ImageInfo, ImageWriteInfo};
use crate::application::ImageManager;
use crate::utils::types::AnyhowExt;

impl ImageManager {
    #[instrument(level = Level::DEBUG, skip_all, fields(manager.folder = ? self.folder, manager.max_bytes_per_user = self.max_bytes_per_user, file = ? file_name), ret(level = Level::DEBUG), err(Display))]
    pub async fn store_image(
        &self,
        access: &DatabaseScopeAccess,
        file_name: &str,
        bytes: Bytes,
    ) -> Result<sql::Thing, StoreImageError> {
        let file_path = Uuid::now_v7().simple().to_string();
        let path = self.folder.join(&file_path);

        let mut decoder = image::io::Reader::new(Cursor::new(&bytes));
        if let Some(format) = ImageFormat::from_path(file_name).ok() {
            decoder.set_format(format);
        }
        decoder = decoder
            .with_guessed_format()
            .anyhow()
            .with_context(|| "guessing image format")?;
        let format = decoder.format().ok_or(StoreImageError::InvalidFormat)?;
        let image = decoder
            .decode()
            .map_err(|_| StoreImageError::InvalidFormat)?;

        let image_info = ImageWriteInfo {
            path: file_path.into(),
            file_name: file_name.to_string(),
            file_type: format.to_mime_type().to_string(),
            file_size: bytes.len() as u64,
            dimensions: Dimensions::from(image.dimensions()),
            owner: access
                .query("SELECT id FROM ONLY $auth;")
                .await
                .anyhow()?
                .take::<Option<sql::Thing>>("id")
                .map_err(|err| StoreImageError::Unauthorized)
                .and_then(|thing| thing.ok_or(StoreImageError::Unauthorized))?,
        };

        debug!(image = ?image_info, "checking image");
        self.check_image(&image_info).await?;

        info!(image = ?image_info, "inserting image");

        let id = self
            .db
            .query("CREATE ONLY image CONTENT $write_data RETURN id;")
            .bind("write_data", &image_info)
            .await
            .anyhow()?
            .take::<Vec<ImageId>>(0)
            .and_then(|values| {
                values
                    .first()
                    .map(|value| value.id.clone())
                    .ok_or(anyhow!("no image created"))
            })?;

        if let Err(err) = tokio::fs::write(path, bytes).await {
            error!(%err, image = ?image_info, "unable to write image to fs");
            self.db
                .query("DELETE $id;")
                .bind("id", &id)
                .await
                .anyhow()?
                .checked()
                .anyhow()?;
            return Err(anyhow!(err.to_string()).into());
        }

        Ok(id)
    }

    /// Returns true if image was deleted, false if there was nothing to remove
    #[instrument(level = Level::DEBUG, skip_all, fields(manager.folder = ? self.folder, id = ? id), ret(level = Level::TRACE))]
    pub async fn delete_image_by_id(&self, id: String) -> bool {
        let path = match self.get_image_path(&id).await {
            Ok(path) => path,
            Err(_) => {
                return false;
            }
        };

        return self.delete_image_by_path(path).await;
    }

    /// Returns true if image was deleted, false if there was nothing to remove
    #[instrument(level = Level::DEBUG, skip_all, fields(manager.folder = ? self.folder, path = ? path), ret(level = Level::TRACE))]
    pub async fn delete_image_by_path(&self, path: impl Into<PathBuf> + Debug) -> bool {
        let path = path.into();
        let mut deleted = false;

        // Delete fs image
        let file = self.folder.join(&path);
        let metadata_result = tokio::fs::metadata(file.clone()).await;
        let requires_delete = match metadata_result {
            Ok(metadata) => metadata.is_file(),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => false,
                _ => {
                    error!(?err, "unable to open file");
                    false
                }
            },
        };

        if requires_delete {
            if let Err(err) = tokio::fs::remove_file(file).await {
                error!(?err, "failed removing image file from fs");
            } else {
                deleted = true;
            }
        }

        // Delete db image
        let db_response = self
            .db
            .query("DELETE image WHERE path = $path;")
            .bind("path", path)
            .await
            .and_then(|responses| responses.checked());
        if let Err(err) = db_response {
            error!(?err, "unable to delete db image");
        } else {
            deleted = true;
        }

        deleted
    }

    #[instrument(level = Level::DEBUG, skip_all, fields(manager.folder = ? self.folder, owner = % owner.borrow()), ret(level = Level::TRACE))]
    pub async fn delete_oldest_image_by_owner(
        &self,
        owner: impl Borrow<sql::Thing>,
    ) -> anyhow::Result<Option<ImageInfo>> {
        let image_info = self.db.query("DELETE ( SELECT * FROM image WHERE owner = $owner ORDER BY time_created LIMIT 1 ) RETURN BEFORE;")
            .bind("owner", owner.borrow())
            .await?
            .take::<Vec<ImageInfo>>(0)?
            .into_iter().next();

        // Delete fs image
        if let Some(image) = &image_info {
            let path = self.folder.join(&image.path);
            if tokio::fs::metadata(&path)
                .await
                .and_then(|metadata| Ok(metadata.is_file()))
                .is_ok_and(|is_file| is_file)
            {
                tokio::fs::remove_file(path).await?
            }
        }

        Ok(image_info)
    }
}

#[derive(Error, Debug)]
pub enum StoreImageError {
    #[error("storage exceeded for user with {amount} bytes, limit is {limit} bytes")]
    StorageExceeded { amount: u64, limit: u64 },
    #[error("provided file was no valid image")]
    InvalidFormat,
    #[error("unauthenticated user")]
    Unauthorized,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

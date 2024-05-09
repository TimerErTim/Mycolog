use std::path::PathBuf;

use anyhow::{anyhow, bail};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::{error, instrument, Level};

use crate::application::database::system::DatabaseScopeAccess;
use crate::application::images::data::{ImageInfo, ImageReadData};
use crate::application::ImageManager;

impl ImageManager {
    pub async fn get_image(
        &self,
        access: &DatabaseScopeAccess,
        id: String,
    ) -> anyhow::Result<ImageReadData> {
        let id = id.trim();
        if id.trim().is_empty() {
            bail!("image manager received empty id");
        }

        // Check for permission
        access
            .query("SELECT VALUE id FROM ONLY type::thing(\"image\", $id);")
            .bind("id", id)
            .await?
            .checked()
            .map_err(|_| anyhow!("image with id `{id}` was not found"))?;

        let info: Option<ImageInfo> = self
            .db
            .query("SELECT * FROM ONLY type::thing('image', $id)")
            .bind("id", id)
            .await?
            .take(0)?;
        let Some(info) = info else {
            bail!("no info for image `{id}`");
        };

        let file = File::open(self.folder.join(&info.path))
            .await
            .map_err(|err| anyhow!("image file unable to open: {err}"))?;

        Ok(ImageReadData {
            info,
            bytes: ReaderStream::new(file),
        })
    }

    #[instrument(level = Level::DEBUG, skip_all, fields(self.folder = ? self.folder, id = ? id))]
    pub(super) async fn get_image_path(&self, id: &str) -> anyhow::Result<PathBuf> {
        let path = self
            .db
            .query("SELECT path FROM ONLY type::thing('image', $id)")
            .bind("id", id)
            .await
            .inspect_err(|err| error!(?err, "unable to execute path extraction query"))?
            .take::<Option<PathBuf>>("path")
            .and_then(|path| path.ok_or(anyhow!("`path` was no valid PathBuf")))
            .inspect_err(|err| error!(?err, "failed to get `path` from query"))?;
        Ok(path)
    }
}

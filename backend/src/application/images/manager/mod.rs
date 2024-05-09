use std::collections::BTreeSet;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use anyhow::{anyhow, bail};
use image::GenericImageView;
use serde::Deserialize;
use surrealdb_core::sql;
use surrealdb_core::sql::Thing;
use tokio::fs::File;
use tokio_util::bytes;
use tokio_util::bytes::Bytes;
use tokio_util::io::ReaderStream;
use tracing::{error, info, instrument, warn, Level};

pub use write::StoreImageError;

use crate::application::database::system::DatabaseScopeAccess;
use crate::application::database::DatabaseRootAccess;
use crate::application::images::data::{Dimensions, ImageCleanInfo, ImageInfo, ImageReadData};
use crate::application::DatabaseSystem;

mod constrain;
pub mod read;
pub mod write;

pub struct ImageManager {
    folder: PathBuf,
    max_bytes_per_user: u64,
    db: DatabaseRootAccess,
}

impl ImageManager {
    pub fn new(
        folder: impl Into<PathBuf>,
        db: DatabaseRootAccess,
        max_bytes_per_user: u64,
    ) -> anyhow::Result<Self> {
        let folder = folder.into();
        if !folder.is_dir() {
            error!(dir = %folder.display(), "cannot create image manager because directory does not exist");
            bail!(
                "image manager was given invalid directory `{}`",
                folder.display()
            );
        }
        if max_bytes_per_user <= 0 {
            error!(
                max_bytes_per_user,
                "cannot create image manager because too few bytes allowed per user"
            );
            bail!(
                "image manager was given invalid max_bytes_per_user `{}`",
                max_bytes_per_user
            );
        }

        Ok(Self {
            folder,
            max_bytes_per_user,
            db,
        })
    }
}

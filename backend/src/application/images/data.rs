use std::borrow::Borrow;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb_core::sql;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

#[derive(Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ImageCleanInfo {
    pub path: String,
    pub id: sql::Thing,
}

impl Borrow<String> for ImageCleanInfo {
    fn borrow(&self) -> &String {
        &self.path
    }
}

pub struct ImageReadData {
    pub info: ImageInfo,
    pub bytes: ReaderStream<File>,
}

#[derive(Deserialize)]
pub struct ImageId {
    pub id: sql::Thing,
}

#[derive(Deserialize, Debug)]
pub struct ImageInfo {
    pub(super) path: PathBuf,
    pub file_type: String,
    pub file_name: String,
    pub file_size: u64,
    pub dimensions: Dimensions,
    pub time_created: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct ImageWriteInfo {
    pub(super) path: PathBuf,
    pub file_type: String,
    pub file_name: String,
    pub file_size: u64,
    pub dimensions: Dimensions,
    pub owner: sql::Thing,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl From<(u32, u32)> for Dimensions {
    fn from((x, y): (u32, u32)) -> Self {
        Self {
            width: x,
            height: y,
        }
    }
}

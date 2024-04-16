use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::anyhow;
use tracing::warn;

use crate::application::EmailManager;

#[derive(Clone)]
pub struct EmailFile {
    pub name: String,
    pub text: Option<String>,
    pub html: Option<String>,
}

pub async fn load_email_files(
    directory: impl Into<PathBuf>,
) -> anyhow::Result<BTreeMap<String, EmailFile>> {
    let mut emails = BTreeMap::new();

    let directory = directory.into();
    let mut read_dir = tokio::fs::read_dir(directory.clone()).await?;
    while let Some(file) = read_dir.next_entry().await? {
        let path = file.path();
        let data: anyhow::Result<_> = try {
            let filename = path
                .file_stem()
                .ok_or(anyhow!("file name is invalid"))?
                .to_str()
                .ok_or(anyhow!("file name no utf-8"))?
                .to_string();
            let extension = path
                .extension()
                .ok_or(anyhow!("extension is invalid"))?
                .to_str()
                .ok_or(anyhow!("extension no utf-8"))?
                .to_string();
            (filename, extension)
        };
        let Ok((filename, extension)) = data else {
            continue;
        };

        let file_entry = emails.entry(filename.clone());
        let field = match extension.as_str() {
            "html" => {
                &mut file_entry
                    .or_insert(EmailFile {
                        name: filename,
                        text: None,
                        html: None,
                    })
                    .html
            }
            "txt" => {
                &mut file_entry
                    .or_insert(EmailFile {
                        name: filename,
                        text: None,
                        html: None,
                    })
                    .text
            }
            _ => {
                continue;
            }
        };
        let file_content = tokio::fs::read_to_string(&path).await?;
        *field = Some(file_content);
    }

    if emails.is_empty() {
        warn!(dir = %directory.display(), "no email templates found");
    }

    Ok(emails)
}

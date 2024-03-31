use std::borrow::Borrow;
use std::path::PathBuf;

use anyhow::anyhow;
use surrealdb_core::sql::Statements;

pub struct SqlFile {
    pub statements: Statements,
}

pub async fn load_surql_file(path: impl Borrow<PathBuf>) -> anyhow::Result<SqlFile> {
    let file_path = path.borrow();
    let content = tokio::fs::read_to_string(file_path).await.map_err(|err| {
        anyhow!(
            "failed reading content of {}: {:?}",
            file_path.display(),
            err
        )
    })?;
    let query = surrealdb_core::sql::parse(&content).map_err(|err| {
        anyhow!(
            "surql file {} contains invalid statements: {:?}",
            file_path.display(),
            err
        )
    })?;

    Ok(SqlFile {
        statements: query.0,
    })
}

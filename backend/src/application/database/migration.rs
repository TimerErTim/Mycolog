use std::ffi::OsString;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{anyhow, bail};
use regex::Regex;
use surrealdb_core::sql::{Datetime, Number, Object, Statements, Value};
use tracing::{debug, error, info, warn};

use crate::application::database::file::load_surql_file;
use crate::application::database::DatabaseRootAccess;
use crate::utils::types::GenericTryInto;

pub struct MigrationManager {
    folder: PathBuf,
    schema_file: SchemaFile,
    migration_files: Vec<MigrationFile>,
}

pub struct SchemaFile {
    statements: Statements,
}

pub struct MigrationFile {
    statements: Statements,
    file_name: String,
    number: usize,
}

impl MigrationManager {
    pub async fn new(folder: impl Into<PathBuf>) -> anyhow::Result<MigrationManager> {
        let folder = folder.into();
        let schema_file = load_schema_file(folder.clone()).await?;
        let migration_files = load_migration_files(folder.clone()).await?;

        info!("found {} migration files", migration_files.len());

        Ok(MigrationManager {
            folder,
            schema_file,
            migration_files,
        })
    }

    pub async fn apply_migrations(&self, db: &DatabaseRootAccess) -> anyhow::Result<u32> {
        let mut db_info: Value = db
            .query("INFO FOR DB")
            .await?
            .take(0)
            .map_err(|_| anyhow!("no info for db"))?;
        let Ok(mut db_info) = db_info.try_into_type::<Object>() else {
            bail!("info for db is not an object");
        };
        let tables = db_info
            .remove("tables")
            .ok_or(anyhow!("no tables inside info for db"))?
            .try_into_type::<Object>()
            .map_err(|_| anyhow!("tables inside info for db is not an object"))?;

        let has_migrations_table = tables.get("migrations").is_some();
        if !has_migrations_table {
            info!("database has no migration content, importing initial schema...");
            db.query("BEGIN TRANSACTION;")
                .query(self.schema_file.statements.clone())
                .query(
                    r#"
                    DEFINE TABLE migration SCHEMAFULL PERMISSIONS NONE;

                    DEFINE FIELD file_name ON TABLE migration TYPE string;
                    DEFINE FIELD number ON TABLE migration TYPE int;
                    DEFINE FIELD exec_time ON TABLE migration TYPE datetime DEFAULT time::now();
                "#,
                )
                .query("COMMIT TRANSACTION;")
                .await?
                .checked()?;
        };

        let mut migrated_count = 0;
        for (index, migration_file) in self.migration_files.iter().enumerate() {
            let file_name = &migration_file.file_name;
            let migration: anyhow::Result<Value> = db
                .query("SELECT exec_time, number FROM ONLY migration WHERE file_name = $migration_file_name LIMIT 1;")
                .bind("migration_file_name", &file_name).await?.take(0);
            if let Ok(migration) = migration
                && migration.is_some()
            {
                let mut migration = migration.try_into_type::<Object>().map_err(|_| {
                    anyhow!("migration data of file {} is not an object", file_name)
                })?;
                let stored_migration_exec_time: Datetime = migration
                    .remove("exec_time")
                    .ok_or(anyhow!(
                        "migration data of file {} has no execution time",
                        file_name
                    ))?
                    .try_into_type::<Datetime>()
                    .map_err(|_| {
                        anyhow!(
                            "execution time of migration file {} is invalid datatype",
                            file_name
                        )
                    })?;
                let stored_migration_number: usize = migration
                    .remove("number")
                    .ok_or(anyhow!(
                        "migration data of file {} has no number",
                        file_name
                    ))?
                    .try_into_type::<Number>()
                    .map_err(|_| {
                        anyhow!("number of migration file {} is invalid datatype", file_name)
                    })?
                    .to_usize();
                debug!(file = %file_name, "migration file has already been imported on {}", stored_migration_exec_time);
                if stored_migration_number != migration_file.number {
                    warn!(file = %file_name, "migration file has number {} on file system but was imported as number {}", migration_file.number, stored_migration_number);
                }
                continue;
            }

            migrated_count += 1;
            db.query("BEGIN TRANSACTION;")
                .query(migration_file.statements.clone())
                .query(format!(
                    "CREATE migration SET file_name = $filename{0}, number = $number{0};",
                    index
                ))
                .bind(format!("filename{}", index), file_name)
                .bind(format!("number{}", index), migration_file.number)
                .query("COMMIT TRANSACTION;")
                .await?
                .checked()?;
        }

        Ok(migrated_count)
    }
}

async fn load_schema_file(folder: impl Into<PathBuf>) -> anyhow::Result<SchemaFile> {
    let folder = folder.into();
    let schema_file = folder.join("schema.surql");
    let sql_file = load_surql_file(schema_file).await?;

    Ok(SchemaFile {
        statements: sql_file.statements,
    })
}

async fn load_migration_files(path: impl Into<PathBuf>) -> anyhow::Result<Vec<MigrationFile>> {
    let folder = path.into();
    let mut migration_files = Vec::new();

    let number_re = Regex::new(r"^(\d+)_")?;
    let mut read_dir = tokio::fs::read_dir(&folder)
        .await
        .map_err(|err| anyhow!("unable to read directory {}: {:?}", folder.display(), err))?;
    while let Ok(Some(file)) = read_dir.next_entry().await {
        let file_path = file.path();
        let file_name = file_path
            .file_name()
            .map(|os_str| os_str.to_str())
            .flatten()
            .ok_or(anyhow!(
                "unable to convert filename of {}",
                file_path.display()
            ))?
            .to_string();
        let file_extension = file_path
            .extension()
            .map(|os_str| os_str.to_str())
            .flatten()
            .ok_or(anyhow!(
                "file {} has no valid extension",
                file_path.display()
            ))?
            .to_string();

        // Ignore base schema file
        if &file_name == "schema.surql" {
            continue;
        }

        if &file_extension != "surql" {
            warn!(
                "migration file {} is no valid SurrealQL file (requires `.surql` extension)",
                file_path.display()
            );
            continue;
        }

        let number_capture = number_re
            .captures(&file_name)
            .map(|captures| captures.get(1))
            .flatten();
        let Some(number_match) = number_capture else {
            warn!("migration file {} does not start with letters and underscore (e.g. `0001_first.surql`)", file_path.display());
            continue;
        };
        let number = usize::from_str(number_match.as_str())?;

        let surql_file = load_surql_file(file_path).await?;

        migration_files.push(MigrationFile {
            statements: surql_file.statements,
            file_name,
            number,
        });
    }
    migration_files.sort_by_key(|migration| migration.number);

    // Checking migration files to ensure numerical consistency
    for (index, file) in migration_files.iter().enumerate() {
        if file.number != index + 1 {
            bail!("migration files do not start at 1 or are more than one apart");
        }
    }

    Ok(migration_files)
}

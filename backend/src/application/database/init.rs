use surrealdb_core::kvs::Datastore;
use tracing::{error, info_span, instrument, Instrument};

use crate::application::database::migration::MigrationManager;
use crate::application::database::system::DatabaseSystem;
use crate::config::MycologConfig;
use crate::context::MycologContext;
use crate::secrets::MycologSecrets;

#[instrument(skip_all)]
pub async fn create_database_system(
    config: &MycologConfig,
    secrets: &MycologSecrets,
) -> anyhow::Result<DatabaseSystem> {
    let db = DatabaseSystem::create(
        "timerertim",
        "mycolog",
        &secrets.db.user(),
        &secrets.db.password(),
    )
    .await?;

    let root_db = db.auth_root();

    async {
        let manager = match MigrationManager::new("migrations/").await {
            Ok(manager) => manager,
            Err(err) => {
                error!(%err, "migrations failed to load due to error");
                return;
            }
        };

        if let Err(err) = manager.apply_migrations(&root_db).await {
            error!(%err, "unable to setup database due to migration error");
        }
    }
    .instrument(info_span!("database_migration"))
    .await;

    Ok(db)
}

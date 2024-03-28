use std::sync::Arc;

use surrealdb_core::dbs::Session;
use surrealdb_core::iam::signin;
use surrealdb_core::kvs::Datastore;
use tracing::error;

use crate::application::DatabaseSystem;

impl DatabaseSystem {
    pub async fn create(ns: &str, db: &str, user: &str, password: &str) -> anyhow::Result<Self> {
        let datastore = if cfg!(feature = "prod-env") {
            Datastore::new("speedb://data/").await?
        } else {
            Datastore::new("memory").await?
        };
        let datastore = datastore
            .with_strict_mode(true)
            .with_auth_enabled(true)
            .with_auth_level_enabled(true);

        let root_session = Session::owner().with_ns(ns).with_db(db);

        datastore.execute(&format!("DEFINE NS {}; DEFINE DB {}; DEFINE USER {} ON DATABASE PASSWORD \"{}\" ROLES EDITOR;", ns, db, user, password), &root_session, None).await?;
        let mut db_session = Session::default();
        signin::db(
            &datastore,
            &mut db_session,
            ns.to_string(),
            db.to_string(),
            user.to_string(),
            password.to_string(),
        )
        .await
        .inspect_err(|err| error!(%err, "database scoped authentification failed"))?;

        Ok(Self::new(datastore, db_session))
    }
}

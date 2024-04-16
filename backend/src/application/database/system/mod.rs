use std::sync::Arc;

use surrealdb_core::dbs::Session;
use surrealdb_core::iam::signin;
use surrealdb_core::kvs::Datastore;
use tracing::{error, warn};

pub use access::AuthToken;
pub use access::DatabaseRootAccess;
pub use access::DatabaseScopeAccess;
pub use opts::{Response, Responses};

use crate::application::database::system::access::{DatabaseAccess, RootAuth, ScopeAuth};

mod access;
mod create;
mod methods;
mod opts;

pub struct DatabaseSystem {
    datastore: Arc<Datastore>,
    db_session: Session,
    scope_template_session: Session,
}

impl DatabaseSystem {
    fn new(datastore: Datastore, root_session: Session) -> Self {
        let mut template_session = Session::default();
        if let Some(ns) = &root_session.ns {
            template_session = template_session.with_ns(ns);
        } else {
            warn!("root session for database has no defined namespace, may lead to scope auth problems later");
        }
        if let Some(db) = &root_session.db {
            template_session = template_session.with_db(db);
        } else {
            warn!("root session for database has no defined database, may lead to scope auth problems later");
        }

        Self {
            datastore: Arc::new(datastore),
            scope_template_session: template_session,
            db_session: root_session,
        }
    }
}

use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter, Write};
use std::sync::Arc;

use anyhow::bail;
use serde::Serialize;
use surrealdb_core::dbs::Session;
use surrealdb_core::iam;
use surrealdb_core::kvs::Datastore;
use surrealdb_core::sql::{to_value, Object, Value};
use tracing::error;

use crate::application::database::system::DatabaseSystem;
use crate::context::MycologContext;

pub type DatabaseRootAccess = DatabaseAccess<RootAuth>;
pub type DatabaseScopeAccess = DatabaseAccess<ScopeAuth>;

impl DatabaseSystem {
    pub fn auth_root(&self) -> DatabaseRootAccess {
        DatabaseAccess {
            datastore: Arc::clone(&self.datastore),
            auth: RootAuth(self.db_session.clone()),
        }
    }

    pub async fn auth_token(
        &self,
        token: impl Into<AuthToken>,
    ) -> anyhow::Result<DatabaseScopeAccess> {
        let mut session = Session::default();
        iam::verify::token(&self.datastore, &mut session, token.into().as_insecure()).await?;

        Ok(DatabaseAccess {
            datastore: Arc::clone(&self.datastore),
            auth: ScopeAuth(session),
        })
    }

    pub async fn signin(&self, scope: &str, vars: impl Serialize) -> anyhow::Result<AuthToken> {
        let value = to_value(vars)?;
        let Value::Object(mut vars) = value else {
            error!(
                ?value,
                "provided vars in database signin were not an object"
            );
            bail!("provided vars in database signin were not an object")
        };
        let maybe_token = surrealdb_core::iam::signin::sc(
            &self.datastore,
            &mut Session::default(),
            self.scope_template_session.ns.as_ref().unwrap().clone(),
            self.scope_template_session.db.as_ref().unwrap().clone(),
            scope.to_string(),
            vars,
        )
        .await?;
        match maybe_token {
            None => bail!("no token generated"),
            Some(token) => Ok(token.into()),
        }
    }

    pub async fn signup(&self, scope: &str, vars: impl Serialize) -> anyhow::Result<AuthToken> {
        let value = to_value(vars)?;
        let Value::Object(mut vars) = value else {
            error!(
                ?value,
                "provided vars in database signup were not an object"
            );
            bail!("provided vars in database signup were not an object")
        };
        let maybe_token = surrealdb_core::iam::signup::sc(
            &self.datastore,
            &mut Session::default(),
            self.scope_template_session.ns.as_ref().unwrap().clone(),
            self.scope_template_session.db.as_ref().unwrap().clone(),
            scope.to_string(),
            vars,
        )
        .await?;
        match maybe_token {
            None => bail!("no token generated"),
            Some(token) => Ok(token.into()),
        }
    }
}

pub struct DatabaseAccess<S: Auth> {
    pub(super) auth: S,
    pub(super) datastore: Arc<Datastore>,
}

pub(super) trait Auth {
    fn as_session(&self) -> &Session;
}

pub struct RootAuth(Session);

impl Auth for RootAuth {
    fn as_session(&self) -> &Session {
        &self.0
    }
}

pub struct ScopeAuth(Session);

impl Auth for ScopeAuth {
    fn as_session(&self) -> &Session {
        &self.0
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct AuthToken(String);

impl Debug for AuthToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScopeToken(?)")
    }
}

impl<S: Into<String>> From<S> for AuthToken {
    fn from(value: S) -> Self {
        AuthToken(value.into())
    }
}

impl AuthToken {
    pub fn as_insecure(&self) -> &str {
        &self.0
    }

    pub fn to_insecure(&self) -> String {
        self.0.clone()
    }
}

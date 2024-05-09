use std::collections::BTreeMap;
use std::fmt::Debug;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;
use std::time::Duration;

use anyhow::{anyhow, bail};
use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb_core::dbs::Response;
use surrealdb_core::err::Error;
use surrealdb_core::sql;
use surrealdb_core::sql::{to_value, Statement, Statements, Value};
use tracing::error;

use crate::application::database::system::access::{Auth, DatabaseAccess};
use crate::application::database::system::opts::{
    IntoStatements, Responses, ResponsesSelector, Stats,
};

impl<S: Auth> DatabaseAccess<S> {
    pub fn query(&self, query: impl IntoStatements) -> Query<S> {
        let mut errors = Vec::new();
        let query_debug = format!("{:?}", &query);
        let statements = match query.into_statements() {
            Ok(statements) => statements,
            Err(err) => {
                error!(
                    querry = query_debug,
                    ?err,
                    "query could not be converted into statements"
                );
                errors.push(err);
                Vec::new()
            }
        };

        Query {
            params: None,
            statements,
            access: &self,
            errors,
        }
    }
}

pub struct Query<'a, S: Auth> {
    params: Option<BTreeMap<String, Value>>,
    statements: Vec<Statement>,
    access: &'a DatabaseAccess<S>,
    errors: Vec<anyhow::Error>,
}

impl<'a, S: Auth> Query<'a, S> {
    pub fn query(mut self, query: impl IntoStatements) -> Self {
        let query_debug = format!("{:?}", &query);
        match query.into_statements() {
            Ok(mut statements) => {
                self.statements.append(&mut statements);
            }
            Err(err) => {
                error!(
                    querry = query_debug,
                    ?err,
                    "query could not be converted into statements"
                );
                self.errors.push(err);
            }
        };
        self
    }

    pub fn bind(mut self, param_name: impl AsRef<str>, value: impl Serialize) -> Self {
        let name = param_name.as_ref();
        match to_value(value) {
            Ok(value) => {
                let params = self.params.get_or_insert_default();
                params.insert(name.to_string(), value);
            }
            Err(err) => {
                error!(?err, "tried to assign invalid value to param `{}`", name);
                self.errors.push(anyhow!(err));
            }
        };
        self
    }
}

impl<'a, S: Auth + Send + 'static> IntoFuture for Query<'a, S> {
    type Output = anyhow::Result<Responses>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>;

    fn into_future(mut self) -> Self::IntoFuture {
        let query = sql::Query(Statements(self.statements));
        let vars = self.params;
        let future = self
            .access
            .datastore
            .process(query, self.access.auth.as_session(), vars);

        Box::pin(async move {
            if !self.errors.is_empty() {
                bail!(self.errors.remove(0))
            }

            match future.await {
                Ok(result) => Ok(Responses(result)),
                Err(err) => Err(anyhow!(err)),
            }
        })
    }
}

impl Responses {
    /// Checks if any of the contained responses expierenced an error. Returns first error encountered.
    pub fn checked(self) -> anyhow::Result<Self> {
        let mut responses = self;
        for response in &mut responses.0 {
            if response.result.is_err() {
                let err = std::mem::replace(&mut response.result, Ok(Value::None));
                bail!(err.unwrap_err());
            }
        }
        Ok(responses)
    }

    pub fn take<R: DeserializeOwned>(
        &mut self,
        index: impl ResponsesSelector<R>,
    ) -> anyhow::Result<R> {
        index.take_from(self)
    }

    pub fn take_with_stats<R: DeserializeOwned>(
        &mut self,
        index: impl ResponsesSelector<R>,
    ) -> anyhow::Result<(Stats, R)> {
        let Some(stats) = index.stats_from(self) else {
            bail!("no stats for the given index");
        };
        let data = index.take_from(self)?;
        Ok((stats, data))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

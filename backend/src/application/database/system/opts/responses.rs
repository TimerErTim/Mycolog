use anyhow::{anyhow, bail};
use serde::de::DeserializeOwned;
use surrealdb_core::dbs::Response;
use surrealdb_core::sql::{from_value, Array, Value};

use crate::application::database::system::opts::stats::{Stats, ToStats};

#[derive(Debug)]
pub struct Responses(pub(in super::super) Vec<Response>);

impl Responses {
    pub fn new(responses: Vec<Response>) -> Self {
        Self(responses)
    }

    pub fn get(&self, index: usize) -> Option<Value> {
        self.0
            .get(index)
            .map(|response| response.result.as_ref().ok().cloned())
            .flatten()
    }
}

/// Represents a way to take a single query result from a list of responses
pub trait ResponsesSelector<R: DeserializeOwned> {
    /// Extracts and deserializes a query result from a query response
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<R>;

    /// Extracts the statistics from a query response
    fn stats_from(&self, responses: &Responses) -> Option<Stats> {
        let respones = &responses.0;
        respones.get(0).map(|x| x.to_stats())
    }
}

impl ResponsesSelector<Value> for usize {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Value> {
        let vec = &mut responses.0;
        if vec.len() <= self {
            bail!("amount of responses <= index, {} <= {}", vec.len(), &self);
        }
        let response = vec.remove(self);
        Ok(response.result?)
    }

    fn stats_from(&self, response: &Responses) -> Option<Stats> {
        response.0.get(*self).map(|x| x.to_stats())
    }
}

impl<T: DeserializeOwned> ResponsesSelector<Option<T>> for usize {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Option<T>> {
        let vec = &mut responses.0;
        if vec.len() <= self {
            bail!("amount of responses <= index, {} <= {}", vec.len(), &self);
        }
        let response = vec.remove(self);
        match response.result? {
            Value::None | Value::Null => Ok(None),
            Value::Object(object) => Ok(Some(
                from_value(Value::Object(object)).map_err(|err| anyhow!(err.error))?,
            )),
            _ => bail!("invalid response type"),
        }
    }

    fn stats_from(&self, response: &Responses) -> Option<Stats> {
        response.0.get(*self).map(|x| x.to_stats())
    }
}

impl ResponsesSelector<Value> for (usize, &str) {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Value> {
        let vec = &mut responses.0;
        if vec.len() <= self.0 {
            bail!("amount of responses <= index, {} <= {}", vec.len(), &self.0);
        }
        let response = vec.remove(self.0);
        let mut result = response.result?;
        if let Value::Object(object) = &mut result {
            let Some(value) = object.remove(self.1) else {
                bail!("provided key \"{}\" not inside response", self.1);
            };
            return Ok(value);
        }
        Ok(result)
    }

    fn stats_from(&self, response: &Responses) -> Option<Stats> {
        response.0.get(self.0).map(|x| x.to_stats())
    }
}

impl<T: DeserializeOwned> ResponsesSelector<Option<T>> for (usize, &str) {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Option<T>> {
        let vec = &mut responses.0;
        if vec.len() <= self.0 {
            bail!("amount of responses <= index, {} <= {}", vec.len(), &self.0);
        }
        let response = vec.remove(self.0);
        let mut result = response.result?;
        if let Value::Object(object) = &mut result {
            let Some(value) = object.remove(self.1) else {
                bail!("provided key \"{}\" not inside response", self.1);
            };
            return match value {
                Value::None | Value::Null => Ok(None),
                Value::Object(object) => Ok(Some(
                    from_value(Value::Object(object)).map_err(|err| anyhow!(err.error))?,
                )),
                _ => bail!("invalid response type"),
            };
        }
        match result {
            Value::None | Value::Null => Ok(None),
            _ => bail!("invalid response type"),
        }
    }

    fn stats_from(&self, response: &Responses) -> Option<Stats> {
        response.0.get(self.0).map(|x| x.to_stats())
    }
}

impl<T: DeserializeOwned> ResponsesSelector<Vec<T>> for usize {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Vec<T>> {
        let vec = &mut responses.0;
        if vec.len() <= self {
            bail!("amount of responses <= index, {} <= {}", vec.len(), &self);
        }
        let response = vec.remove(self);
        let values = match response.result? {
            Value::Array(Array(vec)) => vec,
            vec => vec![vec],
        };
        Ok(from_value(values.into()).map_err(|err| anyhow!(err.error))?)
    }

    fn stats_from(&self, response: &Responses) -> Option<Stats> {
        response.0.get(*self).map(|x| x.to_stats())
    }
}

impl<T: DeserializeOwned> ResponsesSelector<Vec<T>> for (usize, &str) {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Vec<T>> {
        let vec = &mut responses.0;
        if vec.len() <= self.0 {
            bail!("amount of responses <= index, {} <= {}", vec.len(), &self.0);
        }
        let response = vec.remove(self.0);
        if let Value::Object(object) = &mut response.result? {
            let Some(value) = object.remove(self.1) else {
                bail!("provided key \"{}\" not inside response", self.1);
            };
            let values = match value {
                Value::Array(Array(vec)) => vec,
                vec => vec![vec],
            };
            return Ok(from_value(values.into()).map_err(|err| anyhow!(err.error))?);
        }
        bail!("invalid response type");
    }

    fn stats_from(&self, response: &Responses) -> Option<Stats> {
        response.0.get(self.0).map(|x| x.to_stats())
    }
}

impl ResponsesSelector<Value> for &str {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Value> {
        (0, self).take_from(responses)
    }
}

impl<T: DeserializeOwned> ResponsesSelector<Option<T>> for &str {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Option<T>> {
        (0, self).take_from(responses)
    }
}

impl<T: DeserializeOwned> ResponsesSelector<Vec<T>> for &str {
    fn take_from(self, responses: &mut Responses) -> anyhow::Result<Vec<T>> {
        (0, self).take_from(responses)
    }
}

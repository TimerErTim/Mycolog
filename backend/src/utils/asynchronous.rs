use std::future::Future;
use std::panic::{catch_unwind, UnwindSafe};

use anyhow::bail;

pub async fn run_catch<F, T>(future: F) -> anyhow::Result<T>
where
    F: Future<Output = anyhow::Result<T>> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(future).await?
}

pub fn run_catch_blocking<F, T>(func: F) -> anyhow::Result<T>
where
    F: FnOnce() -> anyhow::Result<T> + UnwindSafe,
{
    match catch_unwind(func) {
        Ok(result) => result,
        Err(_err) => {
            bail!("thread encountered panic");
        }
    }
}

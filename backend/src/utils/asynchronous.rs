use std::future::Future;
use std::panic::catch_unwind;

pub async fn run_catch<F, T>(future: F) -> anyhow::Result<T>
where
    F: Future<Output = anyhow::Result<T>> + Send + 'static,
{
    tokio::spawn(future).await?
}

pub fn run_catch_blocking<F, T>(func: F) -> anyhow::Result<T>
where
    F: FnOnce() -> anyhow::Result<T>,
{
    catch_unwind(func)?
}

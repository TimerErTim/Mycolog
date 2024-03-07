use std::process::exit;

use crate::application::run_application;
use crate::cli::parse_arguments;
use crate::shutdown::shutdown;
use crate::startup::startup;

mod application;
mod cli;
mod config;
mod shutdown;
mod startup;
mod utils;

#[tokio::main]
async fn main() {
    let arguments = parse_arguments();

    let mut context = startup(arguments).await;
    let application_code = run_application(&mut context).await;
    let shutdown_code = shutdown(context).await;

    exit(application_code << 4 & shutdown_code);
}

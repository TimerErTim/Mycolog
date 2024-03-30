use std::net::IpAddr;

use clap::Parser;
use tracing::{debug, info};

pub fn parse_arguments() -> MycologArguments {
    debug!("Parsing CLI arguments...");
    let arguments = MycologArguments::parse();
    info!(?arguments, "parsed cli arguments");
    arguments
}

#[derive(Clone, Debug, Parser)]
#[command(version, about)]
pub struct MycologArguments {
    /// The port to listen on.
    #[arg(short, long)]
    pub port: Option<u16>,
    /// The host to bind the server to.
    #[arg(short = 'i', long, value_parser)]
    pub hostname: Option<IpAddr>,
}

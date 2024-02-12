use clap::{crate_description, crate_name, crate_version, Parser};
use std::{net::IpAddr, path::PathBuf};

const DEFAULT_PORT: u16 = 4299;
const DEFAULT_TIMEOUT_DURATION: u64 = 30;

#[derive(Parser)]
#[command(name = crate_name!(), version = crate_version!(), about = crate_description!(), long_about = None)]
pub struct Args {
    /// IP address to connect to
    #[arg(name = "IP")]
    pub ip_address: IpAddr,

    /// Path to file to send
    #[arg(name = "PATH")]
    pub file_path: PathBuf,

    /// Port to connect to
    #[arg(
        short = 'p',
        long = "port",
        name = "PORT",
        default_value_t = DEFAULT_PORT
    )]
    pub port: u16,

    /// Timeout duration in seconds
    #[arg(
        short = 't',
        long = "timeout",
        name = "TIMEOUT",
        default_value_t = DEFAULT_TIMEOUT_DURATION
    )]
    pub timeout_duration: u64,
}

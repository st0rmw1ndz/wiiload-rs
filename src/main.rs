mod args;
mod file;
mod net;

use clap::Parser;
use color_eyre::eyre;
use std::{net::SocketAddr, time::Duration};

fn main() -> eyre::Result<()> {
    enable_ansi_support::enable_ansi_support()?;
    color_eyre::install()?;

    let args = args::Args::parse();
    if !file::is_extension_supported(&args.file_path)? {
        return Err(eyre::eyre!("Unsupported file extension"));
    }
    let (file, metadata, file_name) = file::get_file_info(args.file_path)?;

    let address = SocketAddr::from((args.ip_address, args.port));
    let mut stream = net::connect(&address, Duration::from_secs(args.timeout_duration))?;

    net::send_handshake(&mut stream, file_name.len(), metadata.len())?;
    net::send_file(&mut stream, file, &file_name)?;

    println!("Done!");

    Ok(())
}

use clap::{crate_description, crate_name, crate_version, Parser};
use eyre::Context;
use std::ffi::OsStr;
use std::fs::{File, Metadata};
use std::io::{self, BufReader, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::path::PathBuf;
use std::time::Duration;

const VERSION_MAJOR: u8 = 0;
const VERSION_MINOR: u8 = 5;
const MAGIC_WORD: &str = "HAXX";

const PORT: u16 = 4299;
const TIMEOUT_DURATION: Duration = Duration::from_secs(10);

#[derive(Parser)]
#[command(name = crate_name!(), version = crate_version!(), about = crate_description!(), long_about = None)]
struct Args {
    #[arg(name = "IP", help = "IP address")]
    ip_address: IpAddr,

    #[arg(name = "PATH", help = "Path to file")]
    file_path: PathBuf,
}

fn send_handshake(stream: &mut TcpStream, file_name: &str, metadata: Metadata) -> io::Result<()> {
    let mut data = Vec::new();

    data.extend_from_slice(MAGIC_WORD.as_bytes());
    data.push(VERSION_MAJOR);
    data.push(VERSION_MINOR);
    data.extend_from_slice(&(file_name.len() as u16).to_be_bytes());
    data.extend_from_slice(&(metadata.len() as u32).to_be_bytes());
    data.extend_from_slice(&(metadata.len() as u32).to_be_bytes());

    stream.write_all(&data)?;

    Ok(())
}

fn send_file(stream: &mut TcpStream, file: File, file_name: &str) -> io::Result<()> {
    let mut reader = BufReader::new(file);
    io::copy(&mut reader, stream)?;

    stream.write_all(file_name.as_bytes())?;

    Ok(())
}

fn get_file_info(file_path: PathBuf) -> eyre::Result<(File, Metadata, String)> {
    let file = File::open(&file_path)
        .wrap_err_with(|| format!("Failed to open file at path: {:?}", file_path))?;
    let metadata = file.metadata().wrap_err("Failed to read file metadata")?;
    let file_name = file_path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| eyre::eyre!("Failed to read file name"))?
        .to_owned();

    Ok((file, metadata, file_name.to_owned()))
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
    let (file, metadata, file_name) = get_file_info(args.file_path)?;

    let address = SocketAddr::from((args.ip_address, PORT));
    let mut stream = TcpStream::connect_timeout(&address, TIMEOUT_DURATION)
        .wrap_err("Failed to connect to socket")?;

    send_handshake(&mut stream, &file_name, metadata).wrap_err("Failed to send handshake data")?;
    send_file(&mut stream, file, &file_name).wrap_err("Failed to send file data")?;

    Ok(())
}

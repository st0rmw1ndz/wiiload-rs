use eyre::Context;
use std::{
    fs::File,
    io::{self, BufReader, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

/// The major version number of the Wiiload protocol.
const WIILOAD_VERSION_MAJOR: u8 = 0;
/// The minor version number of the Wiiload protocol.
const WIILOAD_VERSION_MINOR: u8 = 5;
/// The magic word used in the Wiiload protocol to initiate a connection.
const WIILOAD_MAGIC_WORD: &str = "HAXX";

/// Connects to the system using a specified address and timeout duration.
///
/// # Returns
///
/// A Result containing a TcpStream connected to the specified address.
/// Returns an error if the connection fails.
pub fn connect(address: &SocketAddr, timeout: Duration) -> eyre::Result<TcpStream> {
    println!("Connecting to {}...", address);
    let stream = TcpStream::connect_timeout(address, timeout)
        .wrap_err_with(|| format!("Failed to connect to socket at address: {}", address))?;

    Ok(stream)
}

/// Sends the handshake data to the system.
///
/// # Returns
///
/// A Result indicating whether the handshake data was successfully sent.
/// Returns an error if the handshake data could not be sent.
pub fn send_handshake(
    stream: &mut TcpStream,
    file_name_size: usize,
    file_size: u64,
) -> eyre::Result<()> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(WIILOAD_MAGIC_WORD.as_bytes());
    bytes.push(WIILOAD_VERSION_MAJOR);
    bytes.push(WIILOAD_VERSION_MINOR);
    bytes.extend_from_slice(&(file_name_size as u16).to_be_bytes());
    bytes.extend_from_slice(&(file_size as u32).to_be_bytes());
    bytes.extend_from_slice(&(file_size as u32).to_be_bytes());

    println!("Sending handshake...");
    stream
        .write_all(&bytes)
        .wrap_err("Failed to send handshake")?;

    Ok(())
}

/// Sends the file data to the system.
///
/// # Returns
///
/// A Result indicating whether the file data was successfully sent.
/// Returns an error if the file data could not be sent.
pub fn send_file(stream: &mut TcpStream, file: File, file_name: &str) -> eyre::Result<()> {
    println!("Sending file: {}...", file_name);
    let mut reader = BufReader::new(file);
    io::copy(&mut reader, stream).wrap_err("Failed to send file")?;

    stream
        .write_all(file_name.as_bytes())
        .wrap_err("Failed to send file name")?;

    Ok(())
}

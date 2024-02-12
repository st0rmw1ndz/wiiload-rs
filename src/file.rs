use eyre::Context;
use std::{
    ffi::OsStr,
    fs::{File, Metadata},
    path::PathBuf,
};

/// The supported extensions list, executables and an app archive.
const SUPPORTED_EXTENSIONS: [&str; 3] = ["dol", "elf", "zip"];

/// Gets the information about a file.
///
/// # Returns
///
/// A Result containing a tuple of the opened File, its Metadata, and its name as a String.
/// Returns an error if the file cannot be opened, its metadata cannot be read, or its name cannot be read.
pub fn get_file_info(file_path: PathBuf) -> eyre::Result<(File, Metadata, String)> {
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

/// Checks if a file is in the supported extensions list.
///
/// # Returns
///
/// A Result containing a boolean indicating whether the file's extension is in the list of supported extensions.
/// Returns an error if the file's extension cannot be read.
pub fn is_extension_supported(file_path: &PathBuf) -> eyre::Result<bool> {
    let extension = file_path
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| eyre::eyre!("Failed to read file extension"))?;

    Ok(SUPPORTED_EXTENSIONS.contains(&extension))
}

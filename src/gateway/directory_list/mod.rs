use crate::error::Error;

/// Retrieve the directory path based on a given directory key.
#[deprecated(
    note = "Use gateway::retrieve_directory_path::RetrieveDirectoryPathGW, which implements the shared Gateway seam."
)]
pub type RetrieveDirectoryPath = fn(directory_key: &str) -> Result<String, Error>;

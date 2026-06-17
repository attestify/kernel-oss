use crate::error::{Error, Kind};
use std::env;
use std::path::PathBuf;

/// Retrieve the home directory of the current OS user.
pub fn retrieve() -> Result<PathBuf, Error> {
    retrieve_from_env(env::var("HOME").ok(), env::var("USERPROFILE").ok())
}

pub(crate) fn retrieve_from_env(
    home: Option<String>,
    userprofile: Option<String>,
) -> Result<PathBuf, Error> {
    match home.or(userprofile) {
        Some(dir_value) => Ok(PathBuf::from(dir_value)),
        None => Err(Error::for_system(
            Kind::NotFound,
            "Home directory not found".to_string(),
        )),
    }
}

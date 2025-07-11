use crate::error::{Error, Kind};


// A value representing the file name for a file on the virtual file system.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileName {
    value: String
}

impl FileName {

    /// Create a new instance of a [FileNameBuilder] to create a [FileName] instance.
    pub fn builder() -> FileNameBuilder {
        FileNameBuilder::default()
    }

    /// Retrieve the value of the file name
    pub fn value(&self) -> &str {
        &self.value
    }

}

/// Use to build a valid instance of a [FileName]
#[derive(Clone, Default)]
pub struct FileNameBuilder{
    value: Option<String>,
}

impl FileNameBuilder {

    /// Provide a name value for the
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self
    }

    /// Verify and build the [FileName] instance from the provided builder inputs.
    pub fn build(self) -> Result <FileName, Error> {
        let valid_name = validate_name(self.value)?;
        Ok(FileName { value: valid_name })
    }

}


/// Contains all the logic to verify a valid [FileName] value
fn validate_name(value: Option<String>) -> Result<String, Error> {

    let name = value.unwrap_or_default().trim().to_string();

    if name.is_empty() {
        return Err(invalid_input("The file name cannot be empty."))
    }

    if name == "." || name == ".." {
        return Err(invalid_input("The file name cannot be '.' or '..'."))
    }

    if!name.chars().next().unwrap().is_ascii_alphanumeric()
        && name.chars().next().unwrap()!= '-'
        && name.chars().next().unwrap()!= '.'
        && name.chars().next().unwrap()!= '_'
    {
        return Err(invalid_input("The file name must start with a letter, number, '-', '.', or '_'."))
    }

    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' )
    {
        return Err(invalid_input("The file name can only contain alphanumeric characters, '.', '_', or '-'."))
    }

    Ok(name)
}

/// Simple way to prevent duplicative [Error] code since all of are the same [Kind] and for the same [Audience].
fn invalid_input(message: &str) -> Error {
    Error::for_system(Kind::InvalidInput, message.to_string())
}
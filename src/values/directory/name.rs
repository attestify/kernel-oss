use crate::values::specification::name::Name;
use crate::values::Value;

#[derive(Clone, Debug)]
pub struct DirectoryName {
    pub value: String,
}

impl DirectoryName {
    /// Returns the directory name value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Creates a new [`DirectoryName`] from an existing [`Name`] value.
    ///
    /// This method converts the value of the [`Name`] to lowercase.
    pub fn from(name: &Name) -> DirectoryName {
        let value = name.value.to_lowercase();
        DirectoryName { value }
    }
}

impl Value for DirectoryName {
    type ValueType = str;

    fn value(&self) -> &Self::ValueType {
        self.value.as_str()
    }
}

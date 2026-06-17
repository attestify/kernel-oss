use crate::error::{Error, Kind};
use crate::values::specification::description::Description;
use std::collections::HashSet;

/// The [`AdditionalInformation`] allows you to provide information above and beyond the current assurance report data specification. The [`AdditionalInformation`] struct can be used to represent and manage a list of additional information.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct AdditionalInformation {
    list: Vec<Description>,
}

impl AdditionalInformation {
    /// Create a new instance of the [`AdditionalInformationBuilder`].
    ///
    pub fn builder() -> AdditionalInformationBuilder {
        AdditionalInformationBuilder::new()
    }

    /// # Overview
    ///
    /// Get a reference to the list of additional information.
    ///
    pub fn list(&self) -> &[Description] {
        &self.list
    }

    /// # Overview
    ///
    /// Get the number of additional information in the list.
    pub fn count(&self) -> usize {
        self.list.len()
    }
}

/// The [`AdditionalInformationBuilder`] struct is used to create a list of [`AdditionalInformation`] for an assurance report. This builder applies all the validation logic to the list of additional information.
///
#[derive(Clone, Debug)]
pub struct AdditionalInformationBuilder {
    list: Vec<String>,
}

impl AdditionalInformationBuilder {
    /// Create a new instance of the [`AdditionalInformationBuilder`].
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    /// # Overview
    ///
    /// Appends a statement of information to the list of additional information.
    ///
    /// # Arguments
    ///
    /// * `info` - A string slice that holds the information to be added to the list of additional information.
    ///
    /// # Returns
    ///
    /// * A `Result<Self>` is returned with a mutable reference to the current instnace of the [`AdditionalInformationBuilder`] struct.
    ///
    pub fn append(mut self, info: &str) -> Self {
        self.list.push(info.to_string());
        self
    }

    /// # Overview
    ///
    /// Appends a list of information to the list of additional information.
    pub fn extend(mut self, info: &[String]) -> Self {
        self.list.extend(info.iter().cloned());
        self
    }

    /// # Overview
    ///
    /// Attempt to build the list of additional information.
    ///
    /// # Returns
    ///
    /// * A `Result<AdditionalInformation>` is returned with the list of additional information.
    ///
    /// # Errors
    ///
    /// * If the list of additional information contains invalid information, an [`Error`] is returned of kind [`Kind::InvalidInput`] for the audience [`Audience::User`](crate::error::Audience::User).
    ///
    pub fn try_build(self) -> Result<AdditionalInformation, Error> {
        let valid_descriptions = self.validate_information()?;
        Ok(AdditionalInformation {
            list: valid_descriptions,
        })
    }

    fn validate_information(self) -> Result<Vec<Description>, Error> {
        let mut unique_entries = HashSet::new();
        self.list
            .into_iter()
            .filter_map(|info| {
                if unique_entries.insert(info.clone()) {
                    Some(Description::try_from(&info).map_err(|error| {
                        Error::for_user(
                            Kind::InvalidInput,
                            format!(
                                "We could not add the additional information '{}'. {}",
                                info, error.message
                            ),
                        )
                    }))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for AdditionalInformationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

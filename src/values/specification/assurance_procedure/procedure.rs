use crate::error::{Error, Kind};
use crate::values::nrn::NRN;
use crate::values::specification::description::Description;
use crate::values::specification::short_description::ShortDescription;

/// An assurance procedure definition.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Procedure {
    /// The procedure NRN.
    pub nrn: NRN,
    /// The short summary for the procedure.
    pub short: ShortDescription,
    /// The long-form description for the procedure.
    pub description: Description,
}

impl Procedure {
    /// Returns the procedure NRN.
    pub fn nrn(&self) -> &NRN {
        &self.nrn
    }

    /// Returns the short description.
    pub fn short(&self) -> &ShortDescription {
        &self.short
    }

    /// Returns the long description.
    pub fn description(&self) -> &Description {
        &self.description
    }

    /// Creates a validated procedure from string inputs.
    pub fn new(nrn: &str, short_description: &str, long_description: &str) -> Result<Self, Error> {
        let clean_nrn = NRN::new(nrn).map_err(|e| customize_error(e.message.as_str()))?;
        let clean_short = ShortDescription::try_from(short_description).map_err(|e| {
            customize_error(
                format!(
                    "The short description has an issue: '{}'",
                    e.message.as_str()
                )
                .as_str(),
            )
        })?;
        let clean_long = Description::try_from(long_description).map_err(|e| {
            customize_error(
                format!("The description has an issue: '{}'", e.message.as_str()).as_str(),
            )
        })?;

        Ok(Procedure {
            nrn: clean_nrn,
            short: clean_short,
            description: clean_long,
        })
    }
}

fn customize_error(message: &str) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        format!(
            "There is an issue with your procedure information: {}",
            message
        ),
    )
}

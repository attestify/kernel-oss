//! Specification outcome values.

use crate::error;
use crate::error::Error;
use std::fmt::Display;

/// The [`OUTCOME_STRINGS`] contains the list of [`Outcome`]s and their string representations.  This is used for validation of inputs when converting from string to [`Outcome`] enum, and when converting from [`Outcome`] enum to string.
const OUTCOME_STRINGS: &[(Outcome, &str)] = &[
    (Outcome::FAIL, "fail"),
    (Outcome::INCONCLUSIVE, "inconclusive"),
    (Outcome::PASS, "pass"),
    (Outcome::ERROR, "error"),
];

/// Specification outcomes used by assurance reports and procedures.
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
pub enum Outcome {
    /// The check failed.
    FAIL,
    /// The check passed.
    PASS,
    /// The check was inconclusive.
    #[default]
    INCONCLUSIVE,
    /// The check encountered an error.
    ERROR,
}

impl Outcome {
    /// Parses a specification outcome from text.
    pub fn try_from(outcome_value: &str) -> Result<Outcome, Error> {
        if outcome_value.is_empty() {
            return Err(Error::for_user(
                error::Kind::InvalidInput,
                "You have provided an empty Outcome value. Please provide an Outcome value."
                    .to_string(),
            ));
        }
        for (outcome_enum, human_readable_outcome) in OUTCOME_STRINGS {
            if outcome_value.to_lowercase() == *human_readable_outcome {
                return Ok(outcome_enum.clone());
            }
        }
        let list_of_valid_outcome_inputs: Vec<&str> = OUTCOME_STRINGS
            .iter()
            .map(|(_, nid_str)| *nid_str)
            .collect();
        Err(Error::for_user(
            crate::error::Kind::InvalidInput,
            format!(
                "'{}' is not a valid Outcome. Must be one of: [{}].",
                outcome_value,
                list_of_valid_outcome_inputs.join(", ")
            ),
        ))
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (outcome_enum, outcome_str) in OUTCOME_STRINGS {
            if self == outcome_enum {
                return write!(f, "{}", outcome_str);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;

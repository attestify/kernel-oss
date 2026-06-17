use crate::error::{Error, Kind};
use crate::values::specification::assurance_procedure::action::Action;
use crate::values::specification::description::Description;
use crate::values::specification::file_path::FilePath;
use crate::values::specification::name::Name;
use crate::values::specification::short_description::ShortDescription;

/// An assurance procedure activity with expected evidence and actions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Activity {
    /// The activity name.
    pub name: Name,
    /// The short summary for the activity.
    pub short: ShortDescription,
    /// The long-form description for the activity.
    pub description: Description,
    /// The evidence file paths expected for this activity.
    pub expected_evidence: Vec<FilePath>,
    /// The actions that belong to this activity.
    pub actions: Vec<Action>,
}

impl Activity {
    /// Returns the activity name.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Returns the short description.
    pub fn short(&self) -> &ShortDescription {
        &self.short
    }

    /// Returns the long description.
    pub fn description(&self) -> &Description {
        &self.description
    }

    /// Returns the expected evidence file paths.
    pub fn expected_evidence(&self) -> &[FilePath] {
        &self.expected_evidence
    }

    /// Returns the actions associated with this activity.
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Create a new instance of the activity
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the activity
    /// * `short_desc` - A short description of the activity
    /// * `long_desc` - A long description of the activity
    ///
    /// # Returns
    ///
    /// A new instance of the activity or a [`Error`] for [`Audience::User`](crate::error::Audience::User) of [`Kind::InvalidInput`] for any invalid arguments.
    ///
    /// Creates a new activity from the provided strings.
    pub fn new(name: &str, short_desc: &str, long_desc: &str) -> Result<Activity, Error> {
        let valid_name = Name::try_from(name).map_err(|error| {
            custom_error(format!("The name has an issue: {}", error.message).as_str())
        })?;
        let valid_short = ShortDescription::try_from(short_desc).map_err(|error| {
            custom_error(format!("The short description has an issue: {}", error.message).as_str())
        })?;
        let valid_long = Description::try_from(long_desc).map_err(|error| {
            custom_error(format!("The long description has an issue: {}", error.message).as_str())
        })?;

        Ok(Activity {
            name: valid_name,
            short: valid_short,
            description: valid_long,
            expected_evidence: Vec::new(),
            actions: Vec::new(),
        })
    }

    /// Appends an action to the activity and returns a new instance.
    pub fn append_action(self, action: Action) -> Activity {
        let new_actions = self
            .actions
            .clone()
            .into_iter()
            .chain(std::iter::once(action))
            .collect();

        Activity {
            name: self.name,
            short: self.short,
            description: self.description,
            expected_evidence: self.expected_evidence,
            actions: new_actions,
        }
    }

    /// Appends an expected evidence path and returns a new instance.
    pub fn add_expected_evidence(self, file_path: &str) -> Result<Activity, Error> {
        let clean_file_path = FilePath::try_from(file_path).map_err(|error| {
            custom_error(
                format!(
                    "There was an issue adding the expected evidence '{}': {}",
                    file_path, error.message
                )
                .as_str(),
            )
        })?;

        let new_expected_evidence = self
            .expected_evidence
            .into_iter()
            .chain(std::iter::once(clean_file_path))
            .collect();

        Ok(Activity {
            name: self.name,
            short: self.short,
            description: self.description,
            expected_evidence: new_expected_evidence,
            actions: self.actions,
        })
    }

    /// Returns the number of actions in the activity
    /// Returns the number of actions in the activity.
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }
}

/// Creates a user-facing error for assurance procedure activity validation.
pub fn custom_error(message: &str) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        format!(
            "There is an issue with the activity information: {}",
            message
        ),
    )
}

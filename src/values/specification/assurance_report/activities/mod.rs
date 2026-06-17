use crate::error::{Error, Kind};
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::assurance_report::activity::Activity;

/// A collection of assurance report activities.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Activities {
    list: Vec<Activity>,
}

impl Activities {
    /// Creates a new builder for assurance report activities.
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns the activity list.
    pub fn list(&self) -> &Vec<Activity> {
        &self.list
    }

    /// Returns the total number of actions across all activities.
    pub fn action_count(&self) -> usize {
        self.list().iter().map(|activity| activity.count()).sum()
    }

    /// Returns the total number of activities.
    pub fn count(&self) -> usize {
        self.list.len()
    }
}

/// Builder for [`Activities`].
pub struct Builder {
    activities: Vec<Activity>,
    actions: Vec<(String, Action)>,
}

impl Builder {
    /// Create a new instance of the [`Builder`] struct to build an ['Activities'] struct.
    ///
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Builder {
            activities: Vec::new(),
            actions: Vec::new(),
        }
    }

    /// Add an [`Activity`] with a list of existing [`Action`] into the list of activities.
    ///
    /// # Arguments
    ///
    /// * `activity` - A [`Activity`] that holds the name and list of activities to be merged with the list of activities.
    ///
    /// # Returns
    ///
    /// * A mutable reference to the [`Builder`] struct.
    ///
    /// # Notes
    ///
    /// * If the activity name already exists in the list of activities, the activities from the provided activity are added to the existing activity.
    /// * If the activity name does not exist in the list of activities, the provided activity is added to the list of activities.
    ///
    /// Adds an activity to the builder.
    pub fn add_activity(&mut self, activity: &Activity) -> &mut Self {
        self.activities.push(activity.clone());
        self
    }

    /// Add a new [`Action`] to a [`Activity`] by name in the list of activities.
    ///
    /// # Arguments
    ///
    /// * `activity_name` - A string slice that holds the name of the activity to add the activity to.
    /// * `action` - A reference to an [`Action`] struct that holds the details of the activity to be added to the activity.
    ///
    /// # Returns
    ///
    /// * A mutable reference to the [`Builder`] struct.
    ///
    /// # Notes
    ///
    /// * If the activity name already exists in the list of activities, the activity is added to the existing activity.
    /// * If the activity name does not exist in the list of activities, a new activity is created with the provided activity and added to the list of activities.
    ///
    /// Adds an action under an activity name.
    pub fn add_action(&mut self, activity_name: &str, action: &Action) -> &mut Self {
        self.actions
            .push((String::from(activity_name), action.clone()));
        self
    }

    /// Build an instance of the [`Activities`] struct from the list of activities and actions.
    ///
    /// # Returns
    ///
    /// * A new instance of the [`Activities`] struct, or an [`Error`] if the activities could not be created.
    ///
    /// Validates the builder and creates [`Activities`].
    pub fn try_build(&mut self) -> Result<Activities, Error> {
        self.create_activity_from_action_input()?;
        let valid_activities = self.validate_activities()?;
        Ok(Activities {
            list: valid_activities,
        })
    }

    fn create_activity_from_action_input(&mut self) -> Result<(), Error> {
        for (activity_name, action) in &self.actions {
            let activity = Activity::builder()
                .name(activity_name)
                .append_action(action)
                .try_build()
                .map_err(|error| customer_error(&error.message))?;
            self.activities.push(activity);
        }
        Ok(())
    }

    fn validate_activities(&mut self) -> Result<Vec<Activity>, Error> {
        let mut valid_activities: Vec<Activity> = Vec::new();

        for activity in &self.activities {
            if let Some(existing_activity) = valid_activities
                .iter_mut()
                .find(|a| a.name == activity.name)
            {
                let mut builder = Activity::builder();
                let activity_name = activity.name().clone();
                let activity_builder = builder.use_name(&activity_name);

                for existing_action in &existing_activity.actions {
                    activity_builder.append_action(existing_action);
                } // Add the existing actions to the new merged activity
                for action in &activity.actions {
                    activity_builder.append_action(action);
                } // Add the new actions from duplicative activity

                let merged_activity = activity_builder
                    .try_build()
                    .map_err(|error| customer_error(&error.message))?;
                *existing_activity = merged_activity; // Replace the existing activity with the merged activity
            } else {
                valid_activities.push(activity.clone());
            }
        }
        Ok(valid_activities)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;

fn customer_error(message: &str) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        format!("The Activities list could not be created. {}", message),
    )
}

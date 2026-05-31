use crate::values::specification::assurance_report::activities::Activities;
use crate::values::specification::outcome::Outcome;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Summary {
    pub activity_count: u32,
    pub action_count: u32,
    pub actions_run: u32,
    pub pass: u32,
    pub fail: u32,
    pub inconclusive: u32,
    pub outcome: Outcome,
}

impl Summary {
    /// Returns the number of activities in the summarized report.
    pub fn activity_count(&self) -> u32 {
        self.activity_count
    }

    /// Returns the number of actions in the summarized report.
    pub fn action_count(&self) -> u32 {
        self.action_count
    }

    /// Returns the number of actions that ran.
    pub fn actions_run(&self) -> u32 {
        self.actions_run
    }

    /// Returns the number of passing actions.
    pub fn pass(&self) -> u32 {
        self.pass
    }

    /// Returns the number of failing actions.
    pub fn fail(&self) -> u32 {
        self.fail
    }

    /// Returns the number of inconclusive actions.
    pub fn inconclusive(&self) -> u32 {
        self.inconclusive
    }

    /// Returns the summarized outcome.
    pub fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    /// Create a new [`Summary`] from a list of [`Activities`].
    ///
    /// # Arguments
    ///
    /// * `activities` - A reference to a [`Activities`] struct that holds a list of activites.
    ///
    /// # Returns
    ///
    /// * A new [`Summary`] struct.
    ///
    pub fn of(activities: &Activities) -> Summary {
        let (pass, fail, inconclusive, errors) = activities
            .list()
            .iter()
            .flat_map(|activity| activity.actions.iter())
            .fold(
                (0, 0, 0, 0),
                |(pass, fail, inconclusive, errors), activity| match activity.outcome() {
                    Outcome::PASS => (pass + 1, fail, inconclusive, errors),
                    Outcome::FAIL => (pass, fail + 1, inconclusive, errors),
                    Outcome::INCONCLUSIVE => (pass, fail, inconclusive + 1, errors),
                    Outcome::ERROR => (pass, fail, inconclusive, errors + 1),
                },
            );

        Summary {
            activity_count: activities.list().len() as u32,
            action_count: pass + fail + inconclusive + errors,
            actions_run: pass + fail + inconclusive,
            pass,
            fail,
            inconclusive,
            outcome: determine_outcome(pass, fail, inconclusive, errors),
        }
    }
}

fn determine_outcome(pass: u32, fail: u32, inconclusive: u32, errors: u32) -> Outcome {
    match (pass, fail, inconclusive) {
        (_, _, inconclusive) if inconclusive > 0 || errors > 0 => Outcome::INCONCLUSIVE,
        (_, fail, _) if fail > 0 => Outcome::FAIL,
        _ => Outcome::PASS,
    }
}

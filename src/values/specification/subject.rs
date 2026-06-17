//! Specification subject value.

use crate::error::{Error, Kind};
use crate::values::nrn::NRN;
use crate::values::specification::subject_id::SubjectId;

/// A specification subject combining an NRN and subject identifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Subject {
    /// Namespace resource name for the subject.
    pub nrn: NRN,
    /// Subject identifier.
    pub id: SubjectId,
}

impl Subject {
    /// Returns the subject NRN.
    pub fn nrn(&self) -> &NRN {
        &self.nrn
    }

    /// Returns the subject identifier.
    pub fn id(&self) -> &SubjectId {
        &self.id
    }

    /// #Overview
    ///
    /// Attempts to create a new Subject instance.
    ///
    /// # Arguments
    ///
    /// * `nrn` - A string slice representing the NAPE Resource Name of the Subject.
    /// * `id` - A string slice representing the unique identifier of the Subject.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing either a [`Subject`] instance or an [`Error`].
    pub fn try_new(nrn: &str, id: &str) -> Result<Subject, Error> {
        let validated_nrn = match NRN::new(nrn) {
            Ok(nrn) => nrn,
            Err(e) => {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    format!("We were unable to create the Subject: {}", e.message),
                ));
            }
        };
        let validated_subject_id = match SubjectId::new(id) {
            Ok(id) => id,
            Err(e) => {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    format!("We were unable to create the Subject: {}", e.message),
                ));
            }
        };

        Ok(Subject {
            nrn: validated_nrn,
            id: validated_subject_id,
        })
    }
}

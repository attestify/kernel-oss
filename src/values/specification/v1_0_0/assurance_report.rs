use crate::error;
use crate::error::Error;
use crate::values::specification::api_version::APIVersion;
use crate::values::specification::assurance_report::action::Action;
use crate::values::specification::assurance_report::activities::Activities;
use crate::values::specification::assurance_report::activity::Activity;
use crate::values::specification::assurance_report::additional_information::AdditionalInformation;
use crate::values::specification::assurance_report::summary::Summary;
use crate::values::specification::metadata::MetaData;
use crate::values::specification::procedure::Procedure;
use crate::values::specification::subject::Subject;
use crate::values::specification::traits::AssuranceReport;
use std::any::Any;

/// # Overview
///
/// The [`AssuranceReportV1`] is a data structure that represents the NAPE Assurance Report specification v1.0.0.
///
/// Version 1.0.0 assurance report value object.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AssuranceReportV1 {
    metadata: MetaData,
    subject: Subject,
    procedure: Procedure,
    summary: Summary,
    activities: Activities,
    additional_info: AdditionalInformation,
}

impl AssuranceReport for AssuranceReportV1 {
    fn api_version(&self) -> APIVersion {
        APIVersion::new(1, 0, 0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AssuranceReportV1 {
    /// Creates a new builder for an assurance report.
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns the report metadata.
    pub fn metadata(&self) -> &MetaData {
        &self.metadata
    }

    /// Returns the report subject.
    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    /// Returns the report procedure.
    pub fn procedure(&self) -> &Procedure {
        &self.procedure
    }

    /// Returns the report summary.
    pub fn summary(&self) -> &Summary {
        &self.summary
    }

    /// Returns the report activities.
    pub fn activities(&self) -> &Activities {
        &self.activities
    }

    /// Returns the additional information collection.
    pub fn additional_info(&self) -> &AdditionalInformation {
        &self.additional_info
    }
}

/// Builder for [`AssuranceReportV1`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Builder {
    metadata: Option<MetaData>,
    subject: Option<Subject>,
    procedure: Option<Procedure>,
    activities: Option<Activities>,
    summary: Option<Summary>,
    additional_info: Option<AdditionalInformation>,
    metadata_vec: Vec<(String, String)>,
    subject_nrn_str: Option<String>,
    subject_id_str: Option<String>,
    procedure_repo_link_str: Option<String>,
    procedure_directory_str: Option<String>,
    activities_vec: Vec<Activity>,
    actions_vec: Vec<(String, Action)>,
    additional_info_vec: Vec<String>,
}

impl Builder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            metadata: None,
            metadata_vec: Vec::new(),
            subject: None,
            subject_nrn_str: None,
            subject_id_str: None,
            procedure: None,
            procedure_repo_link_str: None,
            procedure_directory_str: None,
            summary: None,
            activities: None,
            activities_vec: Vec::new(),
            actions_vec: Vec::new(),
            additional_info: None,
            additional_info_vec: Vec::new(),
        }
    }

    /// Sets metadata from an existing value object.
    pub fn use_metadata(mut self, metadata: &MetaData) -> Self {
        self.metadata = Some(metadata.clone());
        self
    }

    /// Sets the subject from an existing value object.
    pub fn use_subject(mut self, subject: &Subject) -> Self {
        self.subject = Some(subject.clone());
        self
    }

    /// Sets the procedure from an existing value object.
    pub fn use_procedure(mut self, procedure: &Procedure) -> Self {
        self.procedure = Some(procedure.clone());
        self
    }

    /// Sets the activities from an existing value object.
    pub fn use_activities(mut self, activities: &Activities) -> Self {
        self.activities = Some(activities.clone());
        self
    }

    /// Sets the additional information from an existing value object.
    pub fn use_additional_information(mut self, additional_info: &AdditionalInformation) -> Self {
        self.additional_info = Some(additional_info.clone());
        self
    }

    /// Adds one metadata pair.
    pub fn add_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata_vec.push((key.to_string(), value.to_string()));
        self
    }

    /// Adds many metadata pairs.
    pub fn merge_metadata(mut self, metadata: &[(String, String)]) -> Self {
        self.metadata_vec.extend(metadata.iter().cloned());
        self
    }

    /// Inserts or updates a metadata pair by key.
    pub fn upsert_metadata(mut self, key: &str, value: &str) -> Self {
        for item in self.metadata_vec.iter_mut() {
            if item.0 == key {
                item.1 = value.to_string();
                return self;
            }
        }
        // If the key was not found in the metadata, add a new tuple
        self.metadata_vec.push((key.to_string(), value.to_string()));
        self
    }

    /// Sets the subject NRN from a string.
    pub fn subject_nrn(mut self, nrn: &str) -> Self {
        self.subject_nrn_str = Some(nrn.to_string());
        self
    }

    /// Sets the subject ID from a string.
    pub fn subject_id(mut self, subject_id: &str) -> Self {
        self.subject_id_str = Some(subject_id.to_string());
        self
    }

    /// Sets the procedure repository link from a string.
    pub fn procedure_repository(mut self, repo_link: &str) -> Self {
        self.procedure_repo_link_str = Some(repo_link.to_string());
        self
    }

    /// Sets the procedure directory from a string.
    pub fn procedure_directory(mut self, directory_path: &str) -> Self {
        self.procedure_directory_str = Some(directory_path.to_string());
        self
    }

    /// Adds an activity to the report.
    pub fn add_activity(mut self, activity: &Activity) -> Self {
        self.activities_vec.push(activity.clone());
        self
    }

    /// Adds an action under an activity name.
    pub fn add_action(mut self, activity_name: &str, activity: &Action) -> Self {
        self.actions_vec
            .push((activity_name.to_string(), activity.clone()));
        self
    }

    /// Adds one additional information statement.
    pub fn additional_information(mut self, info: &str) -> Self {
        self.additional_info_vec.push(info.to_string());
        self
    }

    /// Validates the builder and creates an [`AssuranceReportV1`].
    pub fn try_build(self) -> Result<AssuranceReportV1, Error> {
        let metadata = self.build_metadata()?;
        let subject = self.build_subject()?;
        let procedure = self.build_procedure()?;
        let activities = self.build_activities()?;
        let summary = Summary::of(&activities);
        let additional_info = self.build_additional_information()?;

        Ok(AssuranceReportV1 {
            metadata,
            subject,
            procedure,
            summary,
            activities,
            additional_info,
        })
    }

    fn build_metadata(&self) -> Result<MetaData, Error> {
        match &self.metadata {
            Some(metadata) => Ok(metadata.clone()),
            None => {
                let mut metadata = MetaData::default();
                for (key, value) in self.metadata_vec.iter() {
                    metadata.add(key, value).map_err(|e| {
                        customer_error(
                            format!("The Metadata has an issue. {}", e.message.as_str()).as_str(),
                        )
                    })?;
                }
                Ok(metadata)
            }
        }
    }

    fn build_subject(&self) -> Result<Subject, Error> {
        match &self.subject {
            Some(subject) => Ok(subject.clone()),
            None => {
                let nrn = self.subject_nrn_str.as_ref().ok_or(customer_error(
                    "The subject NRN is required, although it was not provided.",
                ))?;
                let subject_id = self.subject_id_str.as_ref().ok_or(customer_error(
                    "The subject ID is required, although it was not provided.",
                ))?;
                Subject::try_new(nrn, subject_id).map_err(|e| {
                    customer_error(
                        format!(
                            "There is an issue with your Subject data. '{}'",
                            e.message.as_str()
                        )
                        .as_str(),
                    )
                })
            }
        }
    }

    fn build_procedure(&self) -> Result<Procedure, Error> {
        match &self.procedure {
            Some(procedure) => Ok(procedure.clone()),
            None => {
                let repo_link = self.procedure_repo_link_str.as_ref().ok_or(customer_error(
                    "The procedure repository link is required, but was not provided.",
                ))?;
                let directory = self.procedure_directory_str.as_ref().ok_or(customer_error(
                    "The procedure directory is required, but was not provided.",
                ))?;
                Procedure::try_new(repo_link, directory).map_err(|e| {
                    customer_error(
                        format!(
                            "There is an issue with your procedure data. {}",
                            e.message.as_str()
                        )
                        .as_str(),
                    )
                })
            }
        }
    }

    fn build_activities(&self) -> Result<Activities, Error> {
        match &self.activities {
            Some(activities) => Ok(activities.clone()),
            None => {
                let mut builder = Activities::builder();
                for activity in self.activities_vec.iter() {
                    builder.add_activity(activity);
                }
                for action in self.actions_vec.iter() {
                    builder.add_action(&action.0, &action.1);
                }
                let activities = builder.try_build().map_err(|e| {
                    customer_error(
                        format!(
                            "There is an issue adding your Activity. {}",
                            e.message.as_str()
                        )
                        .as_str(),
                    )
                })?;
                Ok(activities)
            }
        }
    }

    fn build_additional_information(&self) -> Result<AdditionalInformation, Error> {
        match &self.additional_info {
            Some(additional_info) => Ok(additional_info.clone()),
            None => {
                let additional_info = AdditionalInformation::builder()
                    .extend(&self.additional_info_vec)
                    .try_build()
                    .map_err(|e| {
                        customer_error(
                            format!(
                                "There is an issue adding your Additional Information to the report. {}",
                                e.message.as_str()
                            )
                            .as_str(),
                        )
                    })?;
                Ok(additional_info)
            }
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

fn customer_error(message: &str) -> Error {
    Error::for_user(
        error::Kind::InvalidInput,
        format!("The AssuranceReport could not be created. {}", message),
    )
}

//! Assurance report specification values.

/// Assurance report actions.
pub mod action;
/// Assurance report additional information.
pub mod additional_information;

/// Assurance report activity collections.
pub mod activities;
/// Assurance report activities.
pub mod activity;
/// Signed file values used by assurance reports.
pub mod signed_file;
/// Assurance report summaries.
pub mod summary;

#[cfg(test)]
mod action_tests;
#[cfg(test)]
mod additional_information_tests;

#[cfg(test)]
mod activities_tests;
#[cfg(test)]
mod activity_tests;
#[cfg(test)]
mod signed_file_tests;
#[cfg(test)]
mod summary_tests;

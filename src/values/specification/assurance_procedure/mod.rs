//! Assurance procedure specification values.

/// Assurance procedure actions.
pub mod action;
/// Assurance procedure activity collections.
pub mod activities;
/// Assurance procedure activities.
pub mod activity;
/// Assurance procedure artifacts.
pub mod artifact;
/// Assurance procedure artifact collections.
pub mod artifacts;
/// Assurance procedure definitions.
pub mod procedure;

#[cfg(test)]
mod action_tests;
#[cfg(test)]
mod activities_tests;
#[cfg(test)]
mod activity_tests;
#[cfg(test)]
mod artifact_tests;
#[cfg(test)]
mod artifacts_tests;
#[cfg(test)]
mod procedure_tests;

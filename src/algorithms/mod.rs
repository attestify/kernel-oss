//! Algorithm helpers currently exposed by the kernel.
//!
//! This module is a compatibility surface. Signature algorithms are planned to
//! move to a cryptographic domain crate, and operating-system home-directory
//! behavior is planned to become a kernel driver seam plus driver
//! implementation.

/// Operating-system home-directory helpers.
pub mod os_home_directory;
/// Signature algorithm helpers.
pub mod signature_algorithm;

#[cfg(test)]
mod os_home_directory_tests;
#[cfg(test)]
mod signature_algorithm_tests;

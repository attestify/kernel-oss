//! Verifies the bounded file-name value object.
//!
//! Bounded unit under test:
//! - `FileName`
//!
//! Public interfaces verified:
//! - `FileName::builder().build()`
//! - `FileName::value`
//!
//! Logical paths covered:
//! - valid file names are accepted
//! - leading and trailing whitespace is normalized
//! - empty, dot, dot-dot, invalid-start, and invalid-character names are rejected
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use super::FileName;
use crate::error::Audience;
use crate::error::Kind;
use test_framework_oss::kernel_error_eq;
use test_framework_oss::{is_error, is_ok};

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that valid file names are accepted unchanged.
#[test]
fn valid_names_success() {
    let filename = is_ok!(FileName::builder().value("my_file.txt").build());
    assert_eq!(filename.value(), "my_file.txt");

    let filename = is_ok!(FileName::builder().value("MyFile-123_").build());
    assert_eq!(filename.value(), "MyFile-123_");

    let filename = is_ok!(FileName::builder().value("._-a1").build());
    assert_eq!(filename.value(), "._-a1");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that leading and trailing whitespace is trimmed from file names.
#[test]
fn trims_whitespace_success() {
    let filename = is_ok!(FileName::builder().value("   my_file.txt   ").build());
    assert_eq!(filename.value(), "my_file.txt");
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that empty file names are rejected.
#[test]
fn empty_name_error() {
    let filename = FileName::builder().build();

    is_error!(&filename);
    kernel_error_eq!(
        &filename,
        Kind::InvalidInput,
        Audience::System,
        "The file name cannot be empty."
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that dot and dot-dot file names are rejected.
#[test]
fn dot_or_dot_dot_error() {
    let name1 = FileName::builder().value(".").build();
    is_error!(&name1);
    kernel_error_eq!(
        &name1,
        Kind::InvalidInput,
        Audience::System,
        "The file name cannot be '.' or '..'."
    );

    let name2 = FileName::builder().value("..").build();
    is_error!(&name2);
    kernel_error_eq!(
        &name2,
        Kind::InvalidInput,
        Audience::System,
        "The file name cannot be '.' or '..'."
    )
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that file names starting with unsupported characters are rejected.
#[test]
fn invalid_start_character_error() {
    let name1 = FileName::builder().value("+invalid").build();
    is_error!(&name1);
    kernel_error_eq!(
        &name1,
        Kind::InvalidInput,
        Audience::System,
        "The file name must start with a letter, number, '-', '.', or '_'."
    );
}

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that invalid file-name characters are rejected.
#[test]
fn invalid_characters_error() {
    let name1 = FileName::builder().value("invalid name.txt").build();
    is_error!(&name1);
    kernel_error_eq!(
        &name1,
        Kind::InvalidInput,
        Audience::System,
        "The file name can only contain alphanumeric characters, '.', '_', or '-'."
    );

    let name2 = FileName::builder().value("invalid/").build();
    is_error!(&name2);
    kernel_error_eq!(
        &name2,
        Kind::InvalidInput,
        Audience::System,
        "The file name can only contain alphanumeric characters, '.', '_', or '-'."
    );

    let name3 = FileName::builder().value("inval\\id").build();
    is_error!(&name3);
    kernel_error_eq!(
        &name3,
        Kind::InvalidInput,
        Audience::System,
        "The file name can only contain alphanumeric characters, '.', '_', or '-'."
    );
}

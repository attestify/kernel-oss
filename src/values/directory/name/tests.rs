//! Verifies the directory-name projection from specification names.
//!
//! Bounded unit under test:
//! - `DirectoryName`
//!
//! Public interfaces verified:
//! - `DirectoryName::from`
//!
//! Logical paths covered:
//! - a specification name is normalized to lower-case directory form
//!
//! Requirement validation points:
//! - No requirement validation points are currently supplied.

use crate::values::directory::name::DirectoryName;
use crate::values::specification::name::Name;

/// Requirement validation: No requirement validation point is currently supplied.
///
/// Verifies that a specification name is converted into the expected directory
/// name representation.
#[test]
fn from_success() {
    let name = Name {
        value: String::from("Hello-World"),
    };
    let directory_name = DirectoryName::from(&name);
    assert_eq!(directory_name.value, "hello-world");
}

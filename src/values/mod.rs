pub mod copy_value;
pub mod datetime;
pub mod directory;
pub mod file_system;
pub mod nrn;
pub mod specification;
pub mod strings;
pub mod text;
pub mod uri;
pub mod value;

pub use value::Value;

#[cfg(test)]
mod strings_tests;


use crate::error::Kind;
use crate::error::Audience;
use test_framework_oss::kernel_error_eq;
use test_framework_oss::is_error;
use crate::values::file_system::file_name::FileName;

#[test]
fn valid_name_success() {
    let filename = FileName::builder().value("my_file.txt").build();
    assert!(filename.is_ok());
    assert_eq!(filename.unwrap().value(), "my_file.txt");

    let filename = FileName::builder().value("MyFile-123_").build();
    assert!(filename.is_ok());
    assert_eq!(filename.unwrap().value(), "MyFile-123_");

    let filename = FileName::builder().value("._-a1").build();
    assert!(filename.is_ok());
    assert_eq!(filename.unwrap().value(), "._-a1");
}

#[test]
fn leading_and_trailing_name_success() {
    let filename = FileName::builder().value("   my_file.txt   ").build();
    assert!(filename.is_ok());
    assert_eq!(filename.unwrap().value(), "my_file.txt");
}

#[test]
fn empty_name_error() {

    let filename = FileName::builder().build();

    is_error!(&filename);
    kernel_error_eq!(&filename, Kind::InvalidInput, Audience::System, "The file name cannot be empty.");
}

#[test]
fn dot_or_dot_dot_error() {

    let name1 = FileName::builder().value(".").build();
    is_error!(&name1);
    kernel_error_eq!(&name1, Kind::InvalidInput, Audience::System, "The file name cannot be '.' or '..'.");

    let name2 = FileName::builder().value("..").build();
    is_error!(&name2);
    kernel_error_eq!(&name2, Kind::InvalidInput, Audience::System, "The file name cannot be '.' or '..'.")

}

#[test]
fn invalid_start_character_error() {
    let name1 = FileName::builder().value("+invalid").build();
    is_error!(&name1);
    kernel_error_eq!(&name1, Kind::InvalidInput, Audience::System, "The file name must start with a letter, number, '-', '.', or '_'.");
}

#[test]
fn test_invalid_characters() {

    let name1 = FileName::builder().value("invalid name.txt").build();
    is_error!(&name1);
    kernel_error_eq!(&name1, Kind::InvalidInput, Audience::System, "The file name can only contain alphanumeric characters, '.', '_', or '-'.");
    
    let name2 = FileName::builder().value("invalid/").build();
    is_error!(&name2);
    kernel_error_eq!(&name2, Kind::InvalidInput, Audience::System, "The file name can only contain alphanumeric characters, '.', '_', or '-'.");

    let name3 = FileName::builder().value("inval\\id").build();
    is_error!(&name3);
    kernel_error_eq!(&name3, Kind::InvalidInput, Audience::System, "The file name can only contain alphanumeric characters, '.', '_', or '-'.");

}
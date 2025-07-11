use crate::error::Audience;
use crate::error::Kind;
use attestify_test_framework::kernel_error_eq;
use attestify_test_framework::is_error;
use attestify_test_framework::is_ok;
use crate::values::datetime::DateTime;

#[test]
fn now_success() {
    let datetime = DateTime::builder().now().build();
    is_ok!(&datetime);
    assert!(datetime.unwrap().value() > &&0);
}

#[test]
fn set_at_success() {
    let datetime = DateTime::builder().set_at(1234567890).build();
    is_ok!(&datetime);
    assert_eq!(datetime.unwrap().value(), &1234567890);
}

#[test]
fn now_overrides_set_at_success() {
    let datetime1 = DateTime::builder().set_at(1234567890).now().build();
    is_ok!(&datetime1);
    let value = datetime1.unwrap();
    assert!(&value.value() > &&0);
    assert_ne!(value.value(), &1234567890);

    let datetime2 = DateTime::builder().now().set_at(1234567890).build();
    is_ok!(&datetime2);
    let value2 = datetime2.unwrap();
    assert!(&value2.value() > &&0);
    assert_ne!(value2.value(), &1234567890);
}


#[test]
fn no_value_error() {
    let datetime = DateTime::builder().build();
    is_error!(&datetime);
    kernel_error_eq!(&datetime, Kind::InvalidInput, Audience::System, 
        "A value was not provided for the DateTime, please provide a valid DateTime value.");
}


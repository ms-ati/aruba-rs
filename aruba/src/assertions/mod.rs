use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};
use std::fmt::Debug;

pub fn assert_eq_or_ne<T: PartialEq + Debug>(eq: bool, output: T, expected: T) {
    if eq {
        assert_eq!(output, expected);
    } else {
        assert_ne!(output, expected)
    }
}

pub fn assert_str_eq_or_ne<T: AsRef<str> + PartialEq + Debug>(eq: bool, output: T, expected: T) {
    if eq {
        assert_str_eq!(output, expected);
    } else {
        assert_ne!(output, expected)
    }
}

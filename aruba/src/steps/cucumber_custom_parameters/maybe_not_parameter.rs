use std::str::FromStr;
use cucumber::Parameter;

#[derive(Debug, Default, Eq, Parameter, PartialEq)]
#[param(name = "maybe_not", regex = " | not ")]
pub enum MaybeNotParameter {
    #[default]
    NoNot,
    YesNot,
}

impl FromStr for MaybeNotParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            " "     => Self::NoNot,
            " not " => Self::YesNot,
            invalid => return Err(format!("Invalid cucumber `MaybeNotParameter`: {:?}", invalid)),
        })
    }
}

#[allow(clippy::from_over_into)]
impl Into<bool> for MaybeNotParameter {
    fn into(self) -> bool {
        match self {
            Self::NoNot => true,
            Self::YesNot => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maybe_not_parameter_from_str_table() {
        let table: [(&str, Result<MaybeNotParameter, String>); 3] = [
            (" ",     Ok(MaybeNotParameter::NoNot)),
            (" not ", Ok(MaybeNotParameter::YesNot)),
            ("",      Err("Invalid cucumber `MaybeNotParameter`: \"\"".to_string())),
        ];

        for (input, expected) in &table {
            let result = MaybeNotParameter::from_str(input);
            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn maybe_not_parameter_into_bool_table() {
        let table: [(MaybeNotParameter, bool); 2] = [
            (MaybeNotParameter::NoNot,  true),
            (MaybeNotParameter::YesNot, false),
        ];

        for (input, expected) in table {
            let result: bool = input.into();
            assert_eq!(result, expected);
        }
    }
}

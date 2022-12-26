use cucumber::Parameter;
use std::str::FromStr;

#[derive(Debug, Default, Eq, Parameter, PartialEq)]
#[param(name = "maybe_successfully", regex = " | successfully ")]
pub enum MaybeSuccessfullyParameter {
    #[default]
    NoSuccessfully,
    YesSuccessfully,
}

impl FromStr for MaybeSuccessfullyParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            " " => Self::NoSuccessfully,
            " successfully " => Self::YesSuccessfully,
            invalid => {
                return Err(format!(
                    "Invalid cucumber `MaybeSuccessfullyParameter`: {:?}",
                    invalid
                ))
            }
        })
    }
}

#[allow(clippy::from_over_into)]
impl Into<bool> for MaybeSuccessfullyParameter {
    fn into(self) -> bool {
        match self {
            Self::NoSuccessfully => false,
            Self::YesSuccessfully => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maybe_not_parameter_from_str_table() {
        let table: [(&str, Result<MaybeSuccessfullyParameter, String>); 3] = [
            (" ", Ok(MaybeSuccessfullyParameter::NoSuccessfully)),
            (
                " successfully ",
                Ok(MaybeSuccessfullyParameter::YesSuccessfully),
            ),
            (
                "",
                Err("Invalid cucumber `MaybeSuccessfullyParameter`: \"\"".to_string()),
            ),
        ];

        for (input, expected) in &table {
            let result = MaybeSuccessfullyParameter::from_str(input);
            assert_eq!(&result, expected);
        }
    }

    #[test]
    fn maybe_not_parameter_into_bool_table() {
        let table: [(MaybeSuccessfullyParameter, bool); 2] = [
            (MaybeSuccessfullyParameter::NoSuccessfully, false),
            (MaybeSuccessfullyParameter::YesSuccessfully, true),
        ];

        for (input, expected) in table {
            let result: bool = input.into();
            assert_eq!(result, expected);
        }
    }
}

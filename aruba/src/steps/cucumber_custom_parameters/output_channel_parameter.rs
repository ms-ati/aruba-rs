use cucumber::Parameter;
use std::str::FromStr;

#[derive(Debug, Default, Eq, Parameter, PartialEq)]
#[param(name = "output_channel", regex = "output|stdout|stderr")]
pub enum OutputChannelParameter {
    #[default]
    AllOutput,
    Stdout,
    Stderr,
}

impl FromStr for OutputChannelParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "output" => Self::AllOutput,
            "stdout" => Self::Stdout,
            "stderr" => Self::Stderr,
            invalid => {
                return Err(format!(
                    "Invalid cucumber `OutputChannelParameter`: {:?}",
                    invalid
                ))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maybe_not_parameter_from_str_table() {
        let table: [(&str, Result<OutputChannelParameter, String>); 4] = [
            ("output", Ok(OutputChannelParameter::AllOutput)),
            ("stdout", Ok(OutputChannelParameter::Stdout)),
            ("stderr", Ok(OutputChannelParameter::Stderr)),
            (
                "",
                Err("Invalid cucumber `OutputChannelParameter`: \"\"".to_string()),
            ),
        ];

        for (input, expected) in &table {
            let result = OutputChannelParameter::from_str(input);
            assert_eq!(&result, expected);
        }
    }
}

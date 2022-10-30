use std::str::FromStr;
use cucumber::Parameter;

#[derive(Debug, Default, Eq, Parameter, PartialEq)]
#[param(name = "command_line", regex = "`([^`]+)`")]
pub struct CommandLineParameter(String);

impl FromStr for CommandLineParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.trim_matches('`').to_string()))
    }
}

impl AsRef<str> for CommandLineParameter {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_parameter_from_str_table() {
        let table: [(&str, Result<CommandLineParameter, String>); 3] = [
            ("``",      Ok(CommandLineParameter("".to_string()))),
            ("`ls`",    Ok(CommandLineParameter("ls".to_string()))),
            ("`ls -l`", Ok(CommandLineParameter("ls -l".to_string()))),
        ];

        for (input, expected) in &table {
            let result = CommandLineParameter::from_str(input);
            assert_eq!(&result, expected);
        }
    }
}

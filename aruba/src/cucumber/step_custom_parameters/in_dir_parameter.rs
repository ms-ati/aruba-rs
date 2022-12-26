use cucumber::Parameter;
use std::str::FromStr;

#[derive(Debug, Default, Eq, Parameter, PartialEq)]
#[param(
    name = "maybe_in_dir",
    regex = "| in a temp directory| in the current directory"
)]
pub enum InDirParameter {
    #[default]
    InTempDir,
    InCurrDir,
}

impl FromStr for InDirParameter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "" => Self::InTempDir,
            " in a temp directory" => Self::InTempDir,
            " in the current directory" => Self::InCurrDir,
            invalid => return Err(format!("Invalid cucumber `InDirParameter`: {:?}", invalid)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_dir_parameter_from_str_table() {
        let table: [(&str, Result<InDirParameter, String>); 4] = [
            ("", Ok(InDirParameter::InTempDir)),
            (" in a temp directory", Ok(InDirParameter::InTempDir)),
            (" in the current directory", Ok(InDirParameter::InCurrDir)),
            (
                "boo!",
                Err("Invalid cucumber `InDirParameter`: \"boo!\"".to_string()),
            ),
        ];

        for (input, expected) in &table {
            let result = InDirParameter::from_str(input);
            assert_eq!(&result, expected);
        }
    }
}

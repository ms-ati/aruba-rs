use std::str::FromStr;
use cucumber::Parameter;

#[derive(Debug, Default, Eq, Parameter, PartialEq)]
#[param(name = "maybe_not", regex = "( not)?")]
pub enum MaybeNot {
    #[default]
    NoNot,
    YesNot,
}

impl FromStr for MaybeNot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            ""      => Self::NoNot,
            " not"  => Self::YesNot,
            invalid => return Err(format!("Invalid cucumber parameter `MaybeNot`: {:?}", invalid)),
        })
    }
}

impl Into<bool> for MaybeNot {
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
    fn maybe_not_from_str_table() {
        let table: [(&str, Result<MaybeNot, String>); 3] = [
            ("",     Ok(MaybeNot::NoNot)),
            (" not", Ok(MaybeNot::YesNot)),
            (" ",    Err("Invalid cucumber parameter `MaybeNot`: \" \"".to_string())),
        ];

        for (input, expected) in &table {
            let result = MaybeNot::from_str(input);
            assert_eq!(&result, expected);
        }
    }
}

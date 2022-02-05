use std::{fmt, str::FromStr};

#[derive(Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Other
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Gender::Male => writeln!(f, "Male"),
            &Gender::Female => writeln!(f, "Female"),
            &Gender::Other => writeln!(f, "Other"),
        }
    }
}

impl FromStr for Gender {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Male" => Ok(Gender::Male),
            "Female" => Ok(Gender::Female),
            "Other" => Ok(Gender::Other),
            _ => Err(()),
        }
    }
}

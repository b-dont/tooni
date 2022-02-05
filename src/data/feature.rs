use ::std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub enum FeatureClass {
    Background,
    Racial,
    Class,
    Other,
}

impl Default for FeatureClass {
    fn default() -> Self {
        FeatureClass::Background
    }
}

impl FromStr for FeatureClass {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Background" => Ok(FeatureClass::Background),
            "Racial" => Ok(FeatureClass::Racial),
            "Class" => Ok(FeatureClass::Class),
            "Other" => Ok(FeatureClass::Other),
            _ => Err(()),
        }
    }
}

impl fmt::Display for FeatureClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &FeatureClass::Background => write!(f, "Background"),
            &FeatureClass::Racial => write!(f, "Racial"),
            &FeatureClass::Class => write!(f, "Class"),
            &FeatureClass::Other => write!(f, "Other"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Feature {
    id: Option<i64>,
    class: FeatureClass,
    name: String,
    description: String,
}

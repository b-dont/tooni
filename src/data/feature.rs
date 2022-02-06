use ::std::{fmt, str::FromStr};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug, Clone)]
pub enum FeatureClass {
    Background,
    Racial,
    Class,
    Feat,
    Other,
}

impl FromSql for FeatureClass {
    #[inline]
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<FeatureClass> {
       Ok(FeatureClass::from_str(value.as_str()?).unwrap())
    }
}

impl FromStr for FeatureClass {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Background" => Ok(FeatureClass::Background),
            "Racial" => Ok(FeatureClass::Racial),
            "Class" => Ok(FeatureClass::Class),
            "Feat" => Ok(FeatureClass::Feat),
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
            &FeatureClass::Feat => write!(f, "Feat"),
            &FeatureClass::Other => write!(f, "Other"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Feature {
    pub id: Option<i64>,
    pub class: Option<FeatureClass>,
    pub name: String,
    pub description: String,
}

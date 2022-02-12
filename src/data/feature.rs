use crate::data::character::Model;
use ::std::{fmt, str::FromStr};
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result, Row,
};

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

impl ToSql for FeatureClass {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
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

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, 
            Class: {:#?},
            Name: {}, 
            Description: {}",
            self.id, self.class, self.name, self.description
        )
    }
}

impl Model for Feature {
    fn parameters(&self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(self.id));
        params.push(Box::new(self.class.clone()));
        params.push(Box::new(self.name.clone()));
        params.push(Box::new(self.description.clone()));
        params
    }

    fn build_model(&self, row: &Row) -> Feature
    where
        Self: Sized,
    {
        Feature {
            id: self.id,
            class: self.class.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }

    fn table(&self) -> String {
        todo!();
    }

    fn columns(&self) -> String {
        todo!();
    }

    fn queries(&self) -> String {
        todo!();
    }

    fn values(&self) -> String {
        todo!();
    }
}

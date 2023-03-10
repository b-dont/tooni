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
    pub name: String,
    pub class: Option<FeatureClass>,
    pub description: String,
}

impl Feature {
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, 
            Name: {}, 
            Class: {:#?},
            Description: {}",
            self.id, self.name, self.class, self.description
        )
    }
}

impl Model for Feature {
    fn parameters(&self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(self.id));
        params.push(Box::new(self.name.clone()));
        params.push(Box::new(self.class.clone()));
        params.push(Box::new(self.description.clone()));
        params
    }

    fn build(row: &Row) -> Result<Feature> {
        Ok(Feature {
            id: row.get(0)?,
            name: row.get(1)?,
            class: row.get(2)?,
            description: row.get(3)?,
        })
    }

    fn table() -> String {
        "features".to_string()
    }

    fn columns() -> String {
        "id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        class TEXT NOT NULL,
        description TEXT NOT NULL"
            .to_string()
    }

    fn queries() -> String {
        "id, name, class, description".to_string()
    }

    fn values() -> String {
        "?1, ?2, ?3, ?4".to_string()
    }

    fn id(&self) -> Option<i64> {
        self.id
    }
}

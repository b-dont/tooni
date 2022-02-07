use rusqlite::{
    Result,
    types::{ToSql ,FromSql, FromSqlResult, ToSqlOutput, ValueRef}
};
use std::{fmt, str::FromStr};
use crate::data::character::Model;

#[derive(Debug, Clone)]
// TODO: Consider sub-class in the Enum
// ie. Skill(Acrobatics), Armor(Medium), Weapon(Simple), Tool(DisguiseKit)
pub enum ProficiencyClass {
    Skill,
    Armor,
    Weapon,
    Tool,
}

impl FromSql for ProficiencyClass {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<ProficiencyClass> {
        Ok(ProficiencyClass::from_str(value.as_str()?).unwrap())
    }
}

impl ToSql for ProficiencyClass {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromStr for ProficiencyClass {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Skill" => Ok(ProficiencyClass::Skill),
            "Armor" => Ok(ProficiencyClass::Armor),
            "Weapon" => Ok(ProficiencyClass::Weapon),
            "Tool" => Ok(ProficiencyClass::Tool),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ProficiencyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ProficiencyClass::Skill => write!(f, "Skill"),
            &ProficiencyClass::Armor => write!(f, "Armor"),
            &ProficiencyClass::Weapon => write!(f, "Weapon"),
            &ProficiencyClass::Tool => write!(f, "Tool"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Proficiency {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub class: Option<ProficiencyClass>,
}

impl fmt::Display for Proficiency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, Name: {:#?}, Class: {:#?}",
            self.id, self.name, self.class
        )
    }
}

impl Model for Proficiency {
    fn parameters(&self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(self.id));
        params.push(Box::new(self.name.clone()));
        params.push(Box::new(self.class.clone()));
        params
    }
}

impl Proficiency {
    pub fn new(id: i64, name: String, class: ProficiencyClass) -> Self {
        Self {
            id: Some(id),
            name: Some(name),
            class: Some(class),
        }
    }
}

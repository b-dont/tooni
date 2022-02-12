use crate::data::character::Model;
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result, Row,
};
use std::{fmt, str::FromStr};

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

impl Proficiency {
    pub fn new(&self) -> Self {
        Self::default()
    }
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

    fn build(&self, row: &Row) -> Result<()>
    where
        Self: Sized,
    {
        self.id = row.get(0)?;
        self.name = row.get(1)?;
        self.class = row.get(2)?;

        Ok(())
    }

    fn table(&self) -> String {
        "proficiencies".to_string()
    }

    fn columns(&self) -> String {
        "id INTEGER, 
        name TEXT NOT NULL, 
        class TEXT NOT NULL"
            .to_string()
    }

    fn queries(&self) -> String {
        "id, name, class".to_string()
    }

    fn values(&self) -> String {
        "?1, ?2, ?3".to_string()
    }
}

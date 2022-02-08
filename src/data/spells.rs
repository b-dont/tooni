use crate::data::character::Model;
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result,
};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl FromSql for School {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<School> {
        Ok(School::from_str(value.as_str()?).unwrap())
    }
}

impl ToSql for School {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromStr for School {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Abjuration" => Ok(School::Abjuration),
            "Conjuration" => Ok(School::Conjuration),
            "Divination" => Ok(School::Divination),
            "Enchantment" => Ok(School::Enchantment),
            "Evocation" => Ok(School::Evocation),
            "Illusion" => Ok(School::Illusion),
            "Necromancy" => Ok(School::Necromancy),
            "Transmutation" => Ok(School::Transmutation),
            _ => Err(()),
        }
    }
}

impl fmt::Display for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &School::Abjuration => write!(f, "Abjuration"),
            &School::Conjuration => write!(f, "Conjuration"),
            &School::Divination => write!(f, "Divination"),
            &School::Enchantment => write!(f, "Enchantment"),
            &School::Evocation => write!(f, "Evocation"),
            &School::Illusion => write!(f, "Illusion"),
            &School::Necromancy => write!(f, "Necromancy"),
            &School::Transmutation => write!(f, "Transmutation"),
        }
    }
}

#[derive(Default, Debug, Clone)]
// Enum for school
pub struct Spell {
    pub id: Option<i64>,
    pub name: String,
    pub school: Option<School>,
    pub level: u8,
    pub casting_time: u8,
    pub range: u8,
    pub components: String,
    pub duration: u8,
    pub description: String,
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, 
            Name: {}, 
            School: {:#?},
            Level: {},
            Casting Time: {},
            Range: {},
            Components: {:#?},
            Duration: {},
            Description: {}",
            self.id,
            self.name,
            self.school,
            self.level,
            self.casting_time,
            self.range,
            self.components,
            self.duration,
            self.description
        )
    }
}

impl Model for Spell {
    fn parameters(&self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(self.id));
        params.push(Box::new(self.name.clone()));
        params.push(Box::new(self.school.clone()));
        params.push(Box::new(self.level));
        params.push(Box::new(self.casting_time));
        params.push(Box::new(self.range));
        params.push(Box::new(self.components.clone()));
        params.push(Box::new(self.duration));
        params.push(Box::new(self.description.clone()));
        params
    }

    fn new(&self) -> Spell {
        Self::default()    
    }

    fn return_self(&self) -> Spell 
    where Self : Sized {
        Spell {
            id: self.id,
            name: self.name,
            school: self.school,
            level: self.level,
            casting_time: self.casting_time,
            range: self.range,
            components: self.components,
            duration: self.duration,
            description: self.description
        }
    }
}

impl Spell {
    pub fn new() -> Self {
        Self::default()
    }
}

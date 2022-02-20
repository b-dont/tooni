use crate::data::character::Model;
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result, Row,
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

impl Spell {
    pub fn new() -> Self {
        Self::default()
    }
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

    fn build(row: &Row) -> Result<Self> {
        Ok(Spell {
            id: row.get(0)?,
            name: row.get(1)?,
            school: row.get(2)?,
            level: row.get(3)?,
            casting_time: row.get(4)?,
            range: row.get(5)?,
            components: row.get(6)?,
            duration: row.get(7)?,
            description: row.get(8)?,
        })
    }

    fn junction_ids(&self, table: &str) -> Option<Vec<i64>> {
        None
    }

    fn table() -> String {
        "spells".to_string()
    }

    fn columns() -> String {
        "id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        school TEXT NOT NULL,
        level INTEGER,
        casting_time INTEGER,
        range INTEGER,
        components TEXT NOT NULL,
        duration INTEGER,
        description TEXT NOT NULL"
            .to_string()
    }

    fn queries() -> String {
        "id, name, school, level, casting_time, range, components, duration, description"
            .to_string()
    }

    fn values() -> String {
        "?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9".to_string()
    }

    fn id(&self) -> Option<i64> {
        self.id
    }
}

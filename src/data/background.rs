use crate::data::character::Model;
use crate::data::{feature::Feature, items::Item, language::Language, proficiency::Proficiency};
use rusqlite::{Result, Row, ToSql};
use std::fmt;

#[derive(Default, Clone)]
pub struct Background {
    pub id: Option<i64>,
    pub name: String,
    pub proficiencies: Option<Vec<Proficiency>>,
    pub languages: Option<Vec<Language>>,
    pub starting_equipment: Option<Vec<Item>>,
    pub features: Option<Vec<Feature>>,
    pub personality_traits: Option<Vec<String>>,
    pub ideals: Option<Vec<String>>,
    pub bonds: Option<Vec<String>>,
    pub flaws: Option<Vec<String>>,
}

impl Background {
    pub fn new(&self) -> Self {
        Self::default()
    }
}

impl fmt::Display for Background {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "
            Background: 
            ID: {:#?},
            Name: {},
            ",
            self.id, self.name,
        )
    }
}

impl Model for Background {
    fn parameters(&self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(self.id));
        params.push(Box::new(self.name.clone()));

        for pers_trait in &self.personality_traits.unwrap_or(vec![String::new(); 8]) {
            params.push(Box::new(pers_trait.clone()));
        }
        for ideal in &self.ideals.unwrap_or(vec![String::new(); 6]) {
            params.push(Box::new(ideal.clone()));
        }
        for bond in &self.bonds.unwrap_or(vec![String::new(); 6]) {
            params.push(Box::new(bond.clone()));
        }
        for flaw in &self.flaws.unwrap_or(vec![String::new(); 6]) {
            params.push(Box::new(flaw.clone()));
        }

        params
    }

    fn build(&self, row: &Row) -> Result<()>
    where
        Self: Sized,
    {
        self.id = row.get(0)?;
        self.name = row.get(1)?;
        self.personality_traits = Some(vec![
            row.get(2)?, 
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
            row.get(8)?,
            row.get(9)?,
        ]);
        self.ideals = Some(vec![
            row.get(10)?, 
            row.get(11)?,
            row.get(12)?,
            row.get(13)?,
            row.get(14)?,
            row.get(15)?,
        ]);
        self.bonds = Some(vec![
            row.get(16)?, 
            row.get(17)?,
            row.get(18)?,
            row.get(19)?,
            row.get(20)?,
            row.get(21)?,
        ]);
        self.bonds = Some(vec![
            row.get(22)?, 
            row.get(23)?,
            row.get(24)?,
            row.get(25)?,
            row.get(26)?,
            row.get(27)?,
        ]);

        Ok(())
    }

    fn table(&self) -> String {
        "backgrounds".to_string()
    }

    fn columns(&self) -> String {
        "id INTEGER, 
        name TEXT NOT NULL, 
        personality_traits TEXT NOT NULL, 
        ideals TEXT NOT NULL, 
        bonds TEXT NOT NULL, 
        flaws TEXT NOT NULL"
            .to_string()
    }

    fn queries(&self) -> String {
        "id, name, personality_traits, ideals, bonds, flaws".to_string()
    }

    fn values(&self) -> String {
        "?1, ?2, ?3, ?4, ?5, ?6".to_string()
    }
}

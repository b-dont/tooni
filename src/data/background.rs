use crate::data::character::Model;
use crate::data::{feature::Feature, items::Item, language::Language, proficiency::Proficiency};
use rusqlite::ToSql;
use std::fmt;

#[derive(Default, Clone)]
pub struct Background {
    pub id: Option<i64>,
    pub name: String,
    pub proficiencies: Vec<Proficiency>,
    pub languages: Vec<Language>,
    pub starting_equipment: Vec<Item>,
    pub features: Vec<Feature>,
    pub personality_traits: Vec<String>,
    pub ideals: Vec<String>,
    pub bonds: Vec<String>,
    pub flaws: Vec<String>,
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

        for pers_trait in &self.personality_traits {
            params.push(Box::new(pers_trait.clone()));
        }
        for ideal in &self.ideals {
            params.push(Box::new(ideal.clone()));
        }
        for bond in &self.bonds {
            params.push(Box::new(bond.clone()));
        }
        for flaw in &self.flaws {
            params.push(Box::new(flaw.clone()));
        }

        params
    }
}

impl Background {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn test_background_acolyte() -> Self {
        Background {
            id: None,
            name: "Acolyte".to_string(),
            proficiencies: Vec::new(),
            languages: Vec::new(),
            starting_equipment: Vec::new(),
            features: Vec::new(),
            personality_traits: Vec::new(),
            ideals: Vec::new(),
            bonds: Vec::new(),
            flaws: Vec::new(),
        }
    }
}

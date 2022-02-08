use crate::data::character::Model;
use crate::data::{feature::Feature, items::Item, language::Language, proficiency::Proficiency};
use rusqlite::ToSql;
use std::{fmt, default};

#[derive(Default, Clone)]
pub struct Background {
    pub id: Option<i64>,
    pub name: String,
    pub proficiencies: Option<Vec<Proficiency>>,
    pub languages: Option<Vec<Language>>,
    pub starting_equipment: Option<Vec<Item>>,
    pub features: Option<Vec<Feature>>,
    // TODO: personality_traits needs to always have 
    // an index of 8; the rest need an index of 6
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

    fn return_self(&self) -> Background 
    where Self : Sized {
        Background {
            id: self.id,
            name: self.name.clone(),
            proficiencies: self.proficiencies.clone(),
            languages: self.languages.clone(),
            starting_equipment: self.starting_equipment.clone(),
            features: self.features.clone(),
            personality_traits: self.personality_traits.clone(),
            ideals: self.ideals.clone(),
            bonds: self.bonds.clone(),
            flaws: self.flaws.clone()
        }
    }

    fn new(&self) -> Background {
        Self::default()
    }
}

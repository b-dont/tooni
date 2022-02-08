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
            name: self.name,
            proficiencies: self.proficiencies,
            languages: self.languages,
            starting_equipment: self.starting_equipment,
            features: self.features,
            personality_traits: self.personality_traits,
            ideals: self.ideals,
            bonds: self.bonds,
            flaws: self.flaws
        }
    }

    fn new(&self) -> Background {
        Self::default()
    }
}

impl Background {
    pub fn new() -> Background {
        let mut new_bg = Background {
            id: None,
            name: "".to_string(),
            proficiencies: None,
            languages: None,
            starting_equipment: None,
            features: None,
            personality_traits: vec!["".to_string(); 8],
            ideals: vec!["".to_string(); 6],
            bonds: vec!["".to_string(); 6],
            flaws: vec!["".to_string(); 6],
        };
        new_bg
    }
}

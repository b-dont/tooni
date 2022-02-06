use crate::data::{feature::Feature, items::Item, language::Language, proficiency::Proficiency};
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

impl Background {
    pub fn print_background(&self) {
        println!("{}", self);
        for lang in &self.languages {
            println!("{}", lang);
        }
        for prof in &self.proficiencies {
            println!("{}", prof);
        }
        for item in &self.starting_equipment {
            println!("{}", item);
        }
        for feature in &self.features {
            println!("{}", feature);
        }
        for personality in &self.personality_traits {
            println!("{}", personality);
        }
        for ideal in &self.ideals {
            println!("{}", ideal);
        }
        for bond in &self.bonds {
            println!("{}", bond);
        }
        for flaw in &self.flaws {
            println!("{}", flaw);
        }
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

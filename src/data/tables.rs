use rusqlite::{Row, Result};
use crate::data::{
    character::Model,
    proficiency::Proficiency,
    language::Language,
};

use super::{items::Item, feature::Feature};

#[derive(Debug, Clone)]
pub enum Table {
    ProficiencyTable,
    LanguagesTable,
    ItemsTable,
    FeaturesTable,
    //    SpellsTable,
}

impl Table {
    pub fn name(&self) -> String {
        match self {
            &Table::ProficiencyTable => "proficiencies".to_string(),
            &Table::LanguagesTable => "languages".to_string(),
            &Table::ItemsTable => "items".to_string(),
            &Table::FeaturesTable => "features".to_string(),
        }
    }

    pub fn columns(&self) -> String {
        match self {
                &Table::ProficiencyTable => "
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                class TEXT NOT NULL
                ".to_string(),
                &Table::LanguagesTable => "
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                description TEXT UNIQUE NOT NULL
                ".to_string(),
                &Table::ItemsTable => "
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                class TEXT NOT NULL,
                quantity INTEGER,
                rarity TEXT NOT NULL,
                value INTEGER,
                weight INTEGER,
                properties TEXT NOT NULL,
                description TEXT NOT NULL
                ".to_string(),
                &Table::FeaturesTable => "
                id INTEGER PRIMARY KEY,
                class TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL
                ".to_string(),
        }
    }

    pub fn queries(&self) -> String {
        match self {
            &Table::ProficiencyTable => "id, name, class".to_string(),
            &Table::LanguagesTable => "id, name, description".to_string(),
            &Table::ItemsTable => "id, name, class, quantity, rarity, value, weight, properties, description".to_string(),
            &Table::FeaturesTable => "id, class, name, description".to_string(),
        }
    }

    pub fn values(&self) -> String {
        match self {
            &Table::ProficiencyTable => "?1, ?2, ?3".to_string(),
            &Table::LanguagesTable=> "?1, ?2, ?3".to_string(),
            &Table::ItemsTable => "?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9".to_string(),
            &Table::FeaturesTable => "?1, ?2, ?3, ?4".to_string(),
        }
    }

    pub fn create_model(&self, row: &Row) -> Result<Box<dyn Model>> {
        match self {
            &Table::ProficiencyTable => Ok(Box::new(Proficiency {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?
            })),
            &Table::LanguagesTable => Ok(Box::new(Language {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?
            })),
            &Table::ItemsTable => Ok(Box::new(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                quantity: row.get(3)?,
                rarity: row.get(4)?,
                value: row.get(5)?,
                weight: row.get(6)?,
                properties: row.get(7)?,
                description: row.get(8)?,
            })),
            &Table::FeaturesTable => Ok(Box::new(Feature {
                id: row.get(0)?,
                class: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
            })),
        }
    }
}

#[derive(Debug, Clone)]
pub enum JunctionTable {
    CharacterProfs,
    CharacterSpells,
    CharacterLangs,
    CharacterInvintory,
    CharacterFeatures,
}

impl JunctionTable {
    pub fn name(&self) -> String {
        match self {
            &JunctionTable::CharacterProfs => "character_proficiencies".to_string(),
            &JunctionTable::CharacterSpells => "character_spells".to_string(),
            &JunctionTable::CharacterLangs => "character_languages".to_string(),
            &JunctionTable::CharacterInvintory => "character_invintory".to_string(),
            &JunctionTable::CharacterFeatures => "character_features".to_string(),
        }
    }

    pub fn columns(&self) -> (String, String) {
        match self {
            &JunctionTable::CharacterProfs => ("character".to_string(), "proficiency".to_string()),
            &JunctionTable::CharacterSpells => ("character".to_string(), "spell".to_string()),
            &JunctionTable::CharacterLangs => ("character".to_string(), "language".to_string()),
            &JunctionTable::CharacterInvintory => ("character".to_string(), "item".to_string()),
            &JunctionTable::CharacterFeatures => ("character".to_string(), "feature".to_string()),
        }
    }

    pub fn values(&self) -> String {
        match self {
            _ => "?1, ?2".to_string()
        }
    }
}

use crate::data::{
    character::Model, feature::Feature, items::Item, language::Language, proficiency::Proficiency,
};
use rusqlite::{Result, Row};

use super::background::Background;

#[derive(Debug, Clone)]
pub enum Table {
    BackgroundsTable,
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
            &Table::LanguagesTable   => "languages".to_string(),
            &Table::ItemsTable       => "items".to_string(),
            &Table::FeaturesTable    => "features".to_string(),
            &Table::BackgroundsTable => "backgrounds".to_string(),
        }
    }

    pub fn columns(&self) -> String {
        match self {
            &Table::ProficiencyTable => "
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                class TEXT NOT NULL
                "
            .to_string(),
            &Table::LanguagesTable => "
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                description TEXT UNIQUE NOT NULL
                "
            .to_string(),
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
                "
            .to_string(),
            &Table::FeaturesTable => "
                id INTEGER PRIMARY KEY,
                class TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL
                "
            .to_string(),
            &Table::BackgroundsTable => "
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                personality_trait_1 TEXT NOT NULL,
                personality_trait_2 TEXT NOT NULL,
                personality_trait_3 TEXT NOT NULL,
                personality_trait_4 TEXT NOT NULL,
                personality_trait_5 TEXT NOT NULL,
                personality_trait_6 TEXT NOT NULL,
                personality_trait_7 TEXT NOT NULL,
                personality_trait_8 TEXT NOT NULL,
                ideal_1 TEXT NOT NULL,
                ideal_2 TEXT NOT NULL,
                ideal_3 TEXT NOT NULL,
                ideal_4 TEXT NOT NULL,
                ideal_5 TEXT NOT NULL,
                ideal_6 TEXT NOT NULL,
                bond_1 TEXT NOT NULL,
                bond_2 TEXT NOT NULL,
                bond_3 TEXT NOT NULL,
                bond_4 TEXT NOT NULL,
                bond_5 TEXT NOT NULL,
                bond_6 TEXT NOT NULL,
                flaw_1 TEXT NOT NULL,
                flaw_2 TEXT NOT NULL,
                flaw_3 TEXT NOT NULL,
                flaw_4 TEXT NOT NULL,
                flaw_5 TEXT NOT NULL,
                flaw_6 TEXT NOT NULL
                "
            .to_string(),
        }
    }

    pub fn queries(&self) -> String {
        match self {
            &Table::ProficiencyTable => "id, name, class".to_string(),
            &Table::LanguagesTable   => "id, name, description".to_string(),
            &Table::ItemsTable       => "id, name, class, quantity, rarity, value, weight, properties, description".to_string(),
            &Table::FeaturesTable    => "id, class, name, description".to_string(),
            &Table::BackgroundsTable => {"
                id, 
                name,
                personality_trait_1 TEXT NOT NULL,
                personality_trait_2 TEXT NOT NULL,
                personality_trait_3 TEXT NOT NULL,
                personality_trait_4 TEXT NOT NULL,
                personality_trait_5 TEXT NOT NULL,
                personality_trait_6 TEXT NOT NULL,
                personality_trait_7 TEXT NOT NULL,
                personality_trait_8 TEXT NOT NULL,
                ideal_1 TEXT NOT NULL,
                ideal_2 TEXT NOT NULL,
                ideal_3 TEXT NOT NULL,
                ideal_4 TEXT NOT NULL,
                ideal_5 TEXT NOT NULL,
                ideal_6 TEXT NOT NULL,
                bond_1 TEXT NOT NULL,
                bond_2 TEXT NOT NULL,
                bond_3 TEXT NOT NULL,
                bond_4 TEXT NOT NULL,
                bond_5 TEXT NOT NULL,
                bond_6 TEXT NOT NULL,
                flaw_1 TEXT NOT NULL,
                flaw_2 TEXT NOT NULL,
                flaw_3 TEXT NOT NULL,
                flaw_4 TEXT NOT NULL,
                flaw_5 TEXT NOT NULL,
                flaw_6 TEXT NOT NULL
                ".to_string()
            },
        }
    }

    pub fn values(&self) -> String {
        match self {
            &Table::ProficiencyTable => "?1, ?2, ?3".to_string(),
            &Table::LanguagesTable   => "?1, ?2, ?3".to_string(),
            &Table::ItemsTable       => "?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9".to_string(),
            &Table::FeaturesTable    => "?1, ?2, ?3, ?4".to_string(),
            &Table::BackgroundsTable => {"
                ?1, 
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7,
                ?8,
                ?9,
                ?10,
                ?11,
                ?12,
                ?13,
                ?14,
                ?15,
                ?16,
                ?17,
                ?18,
                ?19,
                ?20,
                ?21,
                ?22,
                ?23,
                ?24,
                ?25,
                ?26,
                ?27,
                ?28
                ".to_string()
            },
        }
    }

    pub fn create_model(&self, row: &Row) -> Result<Box<dyn Model>> {
        match self {
            &Table::ProficiencyTable => Ok(Box::new(Proficiency {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
            })),
            &Table::LanguagesTable => Ok(Box::new(Language {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
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
            &Table::BackgroundsTable => Ok(Box::new(Background {
                id: row.get(0)?,
                name: row.get(1)?,
                proficiencies: Vec::new(),
                languages: Vec::new(),
                starting_equipment: Vec::new(),
                features: Vec::new(),
                personality_traits: vec![
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                    row.get(8)?,
                    row.get(9)?,
                ],
                ideals: vec![
                    row.get(10)?,
                    row.get(11)?,
                    row.get(12)?,
                    row.get(13)?,
                    row.get(14)?,
                    row.get(15)?,
                ],
                bonds: vec![
                    row.get(16)?,
                    row.get(17)?,
                    row.get(18)?,
                    row.get(19)?,
                    row.get(20)?,
                    row.get(21)?,
                ],
                flaws: vec![
                    row.get(22)?,
                    row.get(23)?,
                    row.get(24)?,
                    row.get(25)?,
                    row.get(26)?,
                    row.get(27)?,
                ],
            })),
        }
    }
}

#[derive(Debug, Clone)]
pub enum JunctionTable {
    BackgroundProfs,
    BackgroundLangs,
    BackgroundInvintory,
    BackgroundFeatures,
    BackgroundPersonalityTraits,
    BackgroundIdeals,
    BackgroundBonds,
    BackgroundFlaws,
}

impl JunctionTable {
    pub fn name(&self) -> String {
        match self {
            &JunctionTable::BackgroundProfs             => "background_proficiencies".to_string(),
            &JunctionTable::BackgroundLangs             => "background_languages".to_string(),
            &JunctionTable::BackgroundInvintory         => "background_invintory".to_string(),
            &JunctionTable::BackgroundFeatures          => "background_features".to_string(),
            &JunctionTable::BackgroundPersonalityTraits => "background_personality_traits".to_string(),
            &JunctionTable::BackgroundIdeals            => "background_ideals".to_string(),
            &JunctionTable::BackgroundBonds             => "background_bonds".to_string(),
            &JunctionTable::BackgroundFlaws             => "background_flaws".to_string(),
        }
    }

    pub fn columns(&self) -> (String, String) {
        match self {
            &JunctionTable::BackgroundProfs             => ("background".to_string(), "proficiency".to_string()),
            &JunctionTable::BackgroundLangs             => ("background".to_string(), "language".to_string()),
            &JunctionTable::BackgroundInvintory         => ("background".to_string(), "item".to_string()),
            &JunctionTable::BackgroundFeatures          => ("background".to_string(), "feature".to_string()),
            &JunctionTable::BackgroundPersonalityTraits => ("background".to_string(), "personality_trait".to_string()),
            &JunctionTable::BackgroundIdeals            => ("background".to_string(), "ideal".to_string()),
            &JunctionTable::BackgroundBonds             => ("background".to_string(), "bond".to_string()),
            &JunctionTable::BackgroundFlaws             => ("background".to_string(), "flaw".to_string()),
        }
    }

    pub fn references(&self) -> (String, String) {
        match self {
            &JunctionTable::BackgroundProfs             => ("backgrounds(id)".to_string(), "proficiencies(id)".to_string()),
            &JunctionTable::BackgroundLangs             => ("backgrounds(id)".to_string(), "languages(id)".to_string()),
            &JunctionTable::BackgroundInvintory         => ("backgrounds(id)".to_string(), "items(id)".to_string()),
            &JunctionTable::BackgroundFeatures          => ("backgrounds(id)".to_string(), "features(id)".to_string()),
            &JunctionTable::BackgroundPersonalityTraits => ("backgrounds(id)".to_string(), "personality_traits(id)".to_string()),
            &JunctionTable::BackgroundIdeals            => ("backgrounds(id)".to_string(), "ideals(id)".to_string()),
            &JunctionTable::BackgroundBonds             => ("backgrounds(id)".to_string(), "bonds(id)".to_string()),
            &JunctionTable::BackgroundFlaws             => ("backgrounds(id)".to_string(), "flaws(id)".to_string()),
        }
    }

    pub fn values(&self) -> String {
        match self {
            _ => "?1, ?2".to_string(),
        }
    }
}

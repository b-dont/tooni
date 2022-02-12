use crate::data::{
    character::Model, feature::Feature, items::Item, language::Language, proficiency::Proficiency,
    spells::Spell,
};
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result, Row,
};

use super::background::Background;
use enum_iterator::IntoEnumIterator;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, IntoEnumIterator)]
pub enum Table {
    BackgroundsTable,
    ProficiencyTable,
    LanguagesTable,
    ItemsTable,
    FeaturesTable,
    SpellsTable,
}

impl FromSql for Table {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Table> {
        Ok(Table::from_str(value.as_str()?).unwrap())
    }
}

impl ToSql for Table {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
    }
}

impl FromStr for Table {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "backgrounds" => Ok(Table::BackgroundsTable),
            "proficiencies" => Ok(Table::ProficiencyTable),
            "languages" => Ok(Table::LanguagesTable),
            "items" => Ok(Table::ItemsTable),
            "features" => Ok(Table::FeaturesTable),
            "spells" => Ok(Table::SpellsTable),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Table::BackgroundsTable => write!(f, "backgrounds"),
            &Table::ProficiencyTable => write!(f, "proficiencies"),
            &Table::LanguagesTable => write!(f, "languages"),
            &Table::ItemsTable => write!(f, "items"),
            &Table::FeaturesTable => write!(f, "features"),
            &Table::SpellsTable => write!(f, "spells"),
        }
    }
}

impl Table {
    pub fn name(&self) -> String {
        match self {
            &Table::ProficiencyTable => "proficiencies".to_string(),
            &Table::LanguagesTable => "languages".to_string(),
            &Table::ItemsTable => "items".to_string(),
            &Table::FeaturesTable => "features".to_string(),
            &Table::SpellsTable => "spells".to_string(),
            &Table::BackgroundsTable => "backgrounds".to_string(),
        }
    }

    pub fn has_junctions(&self) -> bool {
        match self {
            &Table::ProficiencyTable => false,
            &Table::LanguagesTable => false,
            &Table::ItemsTable => false,
            &Table::FeaturesTable => false,
            &Table::SpellsTable => false,
            &Table::BackgroundsTable => true,
        }
    }

    pub fn junctions(&self) -> Option<Vec<JunctionTable>> {
        match self {
            &Table::ProficiencyTable => None,
            &Table::LanguagesTable => None,
            &Table::ItemsTable => None,
            &Table::FeaturesTable => None,
            &Table::SpellsTable => None,
            &Table::BackgroundsTable => Some(vec![
                JunctionTable::BackgroundProfs,
                JunctionTable::BackgroundLangs,
                JunctionTable::BackgroundInvintory,
                JunctionTable::BackgroundFeatures,
            ]),
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
            &Table::LanguagesTable => 
            "id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT UNIQUE NOT NULL"
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
            &Table::SpellsTable => 
            "id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            school TEXT NOT NULL,
            level INTEGER,
            casting_time INTEGER,
            range INTEGER,
            components TEXT NOT NULL,
            duration INTEGER,
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
            &Table::LanguagesTable => "id, name, description".to_string(),
            &Table::ItemsTable => {
                "id, name, class, quantity, rarity, value, weight, properties, description"
                    .to_string()
            }
            &Table::FeaturesTable => "id, class, name, description".to_string(),
            &Table::SpellsTable => {
                "id, name, school, level, casting_time, range, components, duration, description"
                    .to_string()
            }
            &Table::BackgroundsTable => "
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
                "
            .to_string(),
        }
    }

    pub fn values(&self) -> String {
        match self {
            &Table::ProficiencyTable => "?1, ?2, ?3".to_string(),
            &Table::LanguagesTable => "?1, ?2, ?3".to_string(),
            &Table::ItemsTable => "?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9".to_string(),
            &Table::FeaturesTable => "?1, ?2, ?3, ?4".to_string(),
            &Table::SpellsTable => "?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9".to_string(),
            &Table::BackgroundsTable => "
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9, ?10, ?11, ?12,
                ?13, ?14, ?15, ?16, ?17, ?18,
                ?19, ?20, ?21, ?22, ?23, ?24,
                ?25, ?26, ?27, ?28
                    "
            .to_string(),
        }
    }
}

#[derive(Debug, Clone, IntoEnumIterator)]
pub enum JunctionTable {
    BackgroundProfs,
    BackgroundLangs,
    BackgroundInvintory,
    BackgroundFeatures,
}

impl JunctionTable {
    pub fn name(&self) -> String {
        match self {
            &JunctionTable::BackgroundProfs => "background_proficiencies".to_string(),
            &JunctionTable::BackgroundLangs => "background_languages".to_string(),
            &JunctionTable::BackgroundInvintory => "background_invintory".to_string(),
            &JunctionTable::BackgroundFeatures => "background_features".to_string(),
        }
    }

    pub fn columns(&self) -> (String, String) {
        match self {
            &JunctionTable::BackgroundProfs => {
                ("background".to_string(), "proficiency".to_string())
            }
            &JunctionTable::BackgroundLangs => ("background".to_string(), "language".to_string()),
            &JunctionTable::BackgroundInvintory => ("background".to_string(), "item".to_string()),
            &JunctionTable::BackgroundFeatures => ("background".to_string(), "feature".to_string()),
        }
    }

    pub fn references(&self) -> (String, String) {
        match self {
            &JunctionTable::BackgroundProfs => {
                ("backgrounds".to_string(), "proficiencies".to_string())
            }
            &JunctionTable::BackgroundLangs => ("backgrounds".to_string(), "languages".to_string()),
            &JunctionTable::BackgroundInvintory => ("backgrounds".to_string(), "items".to_string()),
            &JunctionTable::BackgroundFeatures => {
                ("backgrounds".to_string(), "features".to_string())
            }
        }
    }

    pub fn values(&self) -> String {
        match self {
            _ => "?1, ?2".to_string(),
        }
    }
}

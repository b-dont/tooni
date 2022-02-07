use crate::data::{
    character::{Model, Character},
    proficiency::{Proficiency, ProficiencyClass}
};
use rusqlite::{Row, Result};

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

#[derive(Debug, Clone)]
pub enum Table {
//    CharacterTable,
    ProficiencyTable,
    //    SpellsTable,
    //    LanguagesTable,
    //    ItemsTable,
    //    FeaturesTable,
}

impl Table {
    pub fn name(&self) -> String {
        match self {
//            &Table::CharacterTable => "characters".to_string(),
            &Table::ProficiencyTable => "proficiencies".to_string(),
        }
    }

    pub fn columns(&self) -> String {
        match self {
//            &Table::CharacterTable => "
//                id INTEGER PRIMARY KEY,
//                name TEXT NOT NULL,
//                alignment TEXT NOT NULL,
//                proficiency_bonus INTEGER,
//                passive_perception INTEGER,
//                inspiration INTEGER,
//                speed INTEGER,
//                gender TEXT NOT NULL,
//                weight INTEGER,
//                height INTEGER,
//                age INTEGER,
//                armor_class INTEGER,
//                initiative INTEGER,
//                hit_points INTEGER,
//                temp_hit_points INTEGER,
//                level INTEGER,
//                xp INTEGER,
//                str INTEGER,
//                dex INTEGER,
//                con INTEGER,
//                wis INTEGER,
//                cha INTEGER,
//                str_saving_throw INTEGER,
//                dex_saving_throw INTEGER,
//                con_saving_throw INTEGER,
//                int_saving_throw INTEGER,
//                wis_saving_throw INTEGER,
//                cha_saving_throw INTEGER
//                "
//            .to_string(),
                &Table::ProficiencyTable => "
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                class TEXT NOT NULL
                ".to_string(),
        }
    }

    pub fn table_columns(&self) -> String {
        match self {
            &Table::ProficiencyTable => "id, name, class".to_string(),
        }
    }

    pub fn values(&self) -> String {
        match self {
//            &Table::CharacterTable => "?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29".to_string(),
            &Table::ProficiencyTable => "?1, ?2, ?3".to_string()
        }
    }

    pub fn create_model(&self, row: &Row) -> Result<Box<dyn Model>> {
        match self {
            &Table::ProficiencyTable => Ok(Box::new(Proficiency {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?
            })),
        }
    }
}

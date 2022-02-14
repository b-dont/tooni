use crate::data::character::{Model, Junction};
use crate::data::{feature::Feature, items::Item, language::Language, proficiency::Proficiency};
use rusqlite::{Result, Row, ToSql};
use enum_iterator::IntoEnumIterator;
use std::fmt;

#[derive(Debug, IntoEnumIterator, PartialEq)]
pub enum BackgroundJunctions {
    Profs,
    Langs,
    Inven,
    Feats,
}

impl BackgroundJunctions {
    pub fn tables() -> Vec<String> {
        vec![
            "background_proficiencies".to_string(),
            "background_languages".to_string(),
            "background_inventory".to_string(),
            "background_features".to_string(),
        ]
    }
}

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

        for pers_trait in &self
            .personality_traits
            .clone()
            .unwrap_or(vec![String::new(); 8])
        {
            params.push(Box::new(pers_trait.clone()));
        }
        for ideal in &self.ideals.clone().unwrap_or(vec![String::new(); 6]) {
            params.push(Box::new(ideal.clone()));
        }
        for bond in &self.bonds.clone().unwrap_or(vec![String::new(); 6]) {
            params.push(Box::new(bond.clone()));
        }
        for flaw in &self.flaws.clone().unwrap_or(vec![String::new(); 6]) {
            params.push(Box::new(flaw.clone()));
        }

        params
    }

    fn build(row: &Row) -> Result<Background> {
        Ok(Background {
            id: row.get(0)?,
            name: row.get(1)?,
            personality_traits: Some(vec![
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
                row.get(9)?,
            ]),
            ideals: Some(vec![
                row.get(10)?,
                row.get(11)?,
                row.get(12)?,
                row.get(13)?,
                row.get(14)?,
                row.get(15)?,
            ]),
            bonds: Some(vec![
                row.get(16)?,
                row.get(17)?,
                row.get(18)?,
                row.get(19)?,
                row.get(20)?,
                row.get(21)?,
            ]),
            flaws: Some(vec![
                row.get(22)?,
                row.get(23)?,
                row.get(24)?,
                row.get(25)?,
                row.get(26)?,
                row.get(27)?,
            ]),
            proficiencies: None,
            languages: None,
            features: None,
            starting_equipment: None,
        })
    }

//    fn junctions(&self, table: &str) -> Option<Vec<i64>> {
//        match table {
//            "background_proficiencies" => Some(
//                self.proficiencies
//                    .clone()
//                    .unwrap_or(vec![])
//                    .into_iter()
//                    .map(|prof| prof.id.unwrap())
//                    .collect::<Vec<_>>(),
//            ),
//
//            "background_languages" => Some(
//                self.languages
//                    .clone()
//                    .unwrap_or(vec![])
//                    .into_iter()
//                    .map(|lang| lang.id.unwrap())
//                    .collect::<Vec<_>>(),
//            ),
//
//            "background_inventory" => Some(
//                self.starting_equipment
//                    .clone()
//                    .unwrap_or(vec![])
//                    .into_iter()
//                    .map(|item| item.id.unwrap())
//                    .collect::<Vec<_>>(),
//            ),
//
//            "background_features" => Some(
//                self.features
//                    .clone()
//                    .unwrap_or(vec![])
//                    .into_iter()
//                    .map(|feature| feature.id.unwrap())
//                    .collect::<Vec<_>>(),
//            ),
//            _ => Some(vec![]),
//        }
//    }
//
//    fn references(table: &str) -> Option<(String, String)> {
//        match table {
//            "background_proficiencies" => {
//                Some(("backgrounds".to_string(), "proficiencies".to_string()))
//            }
//            "background_languages" => Some(("backgrounds".to_string(), "languages".to_string())),
//            "background_inventory" => Some(("backgrounds".to_string(), "items".to_string())),
//            "background_features" => Some(("backgrounds".to_string(), "features".to_string())),
//            _ => Some((String::new(), String::new())),
//        }
//    }
//
//    fn junct_columns(table: &str) -> Option<(String, String)> {
//        match table {
//            "background_proficiencies" => {
//                Some(("background".to_string(), "proficiency".to_string()))
//            }
//            "background_languages" => Some(("background".to_string(), "language".to_string())),
//            "background_inventory" => Some(("background".to_string(), "item".to_string())),
//            "background_features" => Some(("background".to_string(), "feature".to_string())),
//            _ => Some((String::new(), String::new())),
//        }
//    }
//
//    fn junct_tables() -> Option<Vec<String>> {
//        Some(vec![
//            "background_proficiencies".to_string(),
//            "background_languages".to_string(),
//            "background_inventory".to_string(),
//            "background_features".to_string(),
//        ])
//    }

    fn table() -> String {
        "backgrounds".to_string()
    }

    fn columns() -> String {
        "id INTEGER, 
        name TEXT NOT NULL, 
        personality_traits TEXT NOT NULL, 
        ideals TEXT NOT NULL, 
        bonds TEXT NOT NULL, 
        flaws TEXT NOT NULL"
            .to_string()
    }

    fn queries() -> String {
        "id, name, personality_traits, ideals, bonds, flaws".to_string()
    }

    fn values() -> String {
        "?1, ?2, ?3, ?4, ?5, ?6".to_string()
    }

    fn id(&self) -> Option<i64> {
        self.id
    }

    fn junction<T: Junction>() -> Option<Vec<T>> {
        Some(BackgroundJunctions::into_enum_iter().collect())
    }
}

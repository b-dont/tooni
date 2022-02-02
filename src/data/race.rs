use crate::data::{feature::Feature, language::Language};

#[derive(Default, Debug, Clone)]
pub struct Race {
    pub id: Option<i64>,
    pub name: String,
    pub languages: Vec<Language>,
    pub skill_proficiencies: Option<Vec<String>>,
    pub armor_proficiencies: Option<Vec<String>>,
    pub weapon_proficiencies: Option<Vec<String>>,
    pub features: Vec<Feature>,
}

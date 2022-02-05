use crate::data::{feature::Feature, language::Language, proficiency::Proficiency};

#[derive(Default, Clone)]
pub struct Race {
    pub id: Option<i64>,
    pub name: String,
    pub languages: Vec<Language>,
    pub proficiencies: Vec<Proficiency>,
    pub features: Vec<Feature>,
}

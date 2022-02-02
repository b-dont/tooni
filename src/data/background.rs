use crate::data::{feature::Feature, items::Item, language::Language};

#[derive(Default, Debug, Clone)]
pub struct Background {
    pub id: Option<i64>,
    pub name: String,
    pub skill_proficiencies: Vec<String>,
    pub languages: Vec<Language>,
    pub starting_equipment: Vec<Item>,
    pub features: Vec<Feature>,
    pub personality_trait: String,
    pub ideal: String,
    pub bond: String,
    pub flaw: String,
}

use std::collections::HashMap;
use crate::data::items::Item;

#[derive(Default, Debug, Clone)]
pub struct Background {
    pub id: Option<u64>,
    pub name: String,
    pub skill_proficiencies: Vec<String>,
    pub languages: Vec<String>,
    pub starting_equipment: Vec<Item>,
    pub features: HashMap<String, String>,
    pub personality_trait: String,
    pub ideal: String,
    pub bond: String,
    pub flaw: String,
}

use std::collections::HashMap;
use crate::data::spells::Spell;

#[derive(Default, Debug, Clone)]
pub struct Race {
    pub id: Option<u64>,
    pub name: String,
    pub languages: Vec<String>,
    pub skill_proficiencies: Option<Vec<String>>,
    pub armor_proficiencies: Option<Vec<String>>,
    pub weapon_proficiencies: Option<Vec<String>>,
    pub features: HashMap<String, String>,
    pub spells: Option<Vec<Spell>>,
}

use crate::data::{feature::Feature, language::Language};

// Enum for saving_throws stats
#[derive(Default, Debug, Clone)]
pub struct Class {
    pub id: Option<i64>,
    pub name: String,
    pub languages: Vec<Language>,
    pub features: Vec<Feature>,
    pub skill_proficiencies: Vec<String>,
    pub armor_proficiencies: Vec<String>,
    pub weapon_proficiencies: Vec<String>,
    pub tool_proficiencies: Vec<String>,
    pub saving_throws: Vec<String>,
    pub hit_dice: (u8, u8),
    pub spells_known: u8,
    pub spell_slots: Option<u8>,
    pub spell_slot_level: Option<u8>,
}

use crate::data::{feature::Feature, language::Language, proficiency::Proficiency};
use std::collections::HashMap;

// Enum for saving_throws stats
#[derive(Default, Clone)]
pub struct Class {
    pub id: Option<i64>,
    pub name: String,
    pub languages: Vec<Language>,
    pub features: Vec<Feature>,
    pub proficiencies: Vec<Proficiency>,
    // TODO: Keys here will share stats Enum:
    pub saving_throws: HashMap<String, bool>,
    pub hit_dice: (u8, u8),
    pub spells_known: Option<u8>,
    pub spell_slots: Option<u8>,
    pub spell_slot_level: Option<u8>,
}

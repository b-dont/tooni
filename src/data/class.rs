use crate::data::{feature::Feature, language::Language, proficiency::Proficiency, stats::Stats};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Class {
    pub id: Option<i64>,
    pub name: String,
    pub languages: Vec<Language>,
    pub features: Vec<Feature>,
    pub proficiencies: Vec<Proficiency>,
    pub saving_throws: HashMap<Stats, bool>,
    pub hit_dice: (u8, u8),
    pub spells_known: Option<u8>,
    pub spell_slots: Option<u8>,
    pub spell_slot_level: Option<u8>,
}

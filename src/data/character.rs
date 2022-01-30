use std::collections::HashMap;
use crate::data::{
    spells::Spell,
    items::Item,
    race::Race,
    class::Class,
    background::Background,
};

#[derive(Default, Clone)]
// The character struct is used by both the database interface
// and the TUI interface to save and render character data.
// Each of the struct's elements is public and represents a piece of
// character data. The only impl is `new()` which will call `default()`.
//
// TODO: As of right now, there has been no need for additional impls
// on this struct. This may change with additional features.
// The struct does not currently contain all needed elements of data,
// and consideration is needed for possible enums or additional structs
// for data such as race, class, background, and others.

// Character is a full, struct representation of a D&D character.
// This struct is used by the CharacterScreen state to display and
// interact with all elements of the saved character. Any changes
// that are made in to the character the CharacterScreen state are
// saved dynamically to the struct (at least that's the plan).
pub struct Character {
    pub id: Option<i64>,
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub background: Background,
    pub alignment: String,
    pub stats: HashMap<String, u8>,
    pub proficiencies: HashMap<String, bool>,
    pub proficiency_bonus: u8,
    pub passive_perception: u8,
    pub inspiration: bool,
    pub languages: Vec<String>,
    pub equipment: Vec<Item>,
    pub spells: Option<Vec<Spell>>,
    pub speed: u8,
    pub gender: String,
    pub height: u8,
    pub weight: u8,
    pub age: u8,
    pub armor_class: u8,
    pub initiative: u8,
    pub hit_points: u16,
    pub temp_hit_points: u16,
    pub level: u8,
    pub xp: u64,
}

impl Character {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Clone)]
// A SavedCharacter is a lightweight character representation
// that holds only the most basic information. This is used
// by the SelectScreen state to display a menu of all currently-saved
// characters in the database.
pub struct SavedCharacter {
    pub id: Option<u64>,
    pub name: String,
    pub race: String,
    pub class: String,
}

impl SavedCharacter {
    pub fn new() -> Self {
        Self::default()
    }
}

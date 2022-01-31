use std::collections::HashMap;
use std::fmt;
use crate::data::{
    spells::Spell,
    items::Item,
    race::Race,
    class::Class,
    background::Background,
    language::Language,
};

#[derive(Default, Clone)]
// The character struct is used by both the database interface
// and the TUI interface to save and render character data.
// Each of the struct's elements is public and represents a piece of
// character data.
//
// TODO: Change alignment, stats, gender to Enums; impl Default;
// TODO: Consider sets instead of Vecs for data structures
pub struct Character {
    pub id: Option<i64>,
    pub name: String,
//    pub race: Race,
//    pub class: Class,
//    pub background: Background,
    pub alignment: String,
//    pub stats: HashMap<String, u8>,
//    pub proficiencies: HashMap<String, bool>,
    pub proficiency_bonus: u8,
    pub passive_perception: u8,
    pub inspiration: bool,
    pub languages: Vec<Language>,
//    pub equipment: Vec<Item>,
//    pub spells: Option<Vec<Spell>>,
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

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       writeln!(f, "
           {:#?}\n
           {}
           {}
           {}
           {}
           {}
           {:#?}
           {}
           {}
           {}
           {}
           {}
           {}
           {}
           {}
           {}
           {}
           {}", 
           self.id,
           self.name,
           self.alignment,
           self.proficiency_bonus,
           self.passive_perception,
           self.inspiration,
           self.languages,
           self.speed,
           self.gender,
           self.height,
           self.weight,
           self.age,
           self.armor_class,
           self.initiative,
           self.hit_points,
           self.temp_hit_points,
           self.level,
           self.xp,
           )
    }
}

impl Character {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn test_character_frank() -> Self {
        let mut frank = Character { 
            id: None, 
            name: "Frank".to_string(), 
            alignment: "Neutral".to_string(), 
            proficiency_bonus: 2, 
            passive_perception: 12, 
            inspiration: false, 
            languages: Vec::new(),
            speed: 30, 
            gender: "Male".to_string(), 
            height: 6, 
            weight: 100, 
            age: 30, 
            armor_class: 13, 
            initiative: 3, 
            hit_points: 10, 
            temp_hit_points: 0, 
            level: 1, 
            xp: 0 
        };

        frank
    }

    pub fn test_character_kevin() -> Self {
        let mut kevin = Character { 
            id: None, 
            name: "Kevin".to_string(), 
            alignment: "Evil".to_string(), 
            proficiency_bonus: 2, 
            passive_perception: 12, 
            inspiration: false, 
            languages: Vec::new(),
            speed: 30, 
            gender: "Female".to_string(), 
            height: 6, 
            weight: 100, 
            age: 30, 
            armor_class: 13, 
            initiative: 3, 
            hit_points: 10, 
            temp_hit_points: 0, 
            level: 1, 
            xp: 0 
        };

        kevin
    }
}

#[derive(Default, Clone)]
// A SavedCharacter is a lightweight character representation
// that holds only the most basic information. This is used
// by the SelectScreen state to display a menu of all currently-saved
// characters in the database.
pub struct SavedCharacter {
    pub id: Option<i64>,
    pub name: String,
    pub race: String,
    pub class: String,
}

impl SavedCharacter {
    pub fn new() -> Self {
        Self::default()
    }
}

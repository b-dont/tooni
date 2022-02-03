use crate::data::{
    background::Background, class::Class, items::Item, language::Language,
    proficiency::Proficiency, race::Race, spells::Spell,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone)]
// The character struct is used by both the database interface
// and the TUI interface to save and render character data.
// Each of the struct's elements is public and represents a piece of
// character data.
//
// TODO: Change alignment, stats, gender to Enums; impl Default;
// TODO: Also need to change "class" for proficiencies, items, features to Enums.
// TODO: Spell schools need to be Enums.
// TODO: Consider sets instead of Vecs for data structures
//
pub struct Character {
    pub id: Option<i64>,
    pub name: String,
    //    pub race: Race,
    //    pub class: Class,
    //    pub background: Background,
    pub alignment: String,
    pub stats: HashMap<String, u8>,
    pub proficiencies: Vec<Proficiency>,
    pub saving_throws: HashMap<String, bool>,
    pub proficiency_bonus: u8,
    pub passive_perception: u8,
    pub inspiration: bool,
    pub languages: Vec<Language>,
    pub invintory: Vec<Item>,
    //    pub spells: Vec<Spell>,
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
        writeln!(
            f,
            "
           ID: {:#?}\n
           Name: {}
           Alignment: {}
           Prof Bonus: {}
           Passive Perception: {}
           Inspiration: {}
           Speed: {}
           Gender: {}
           Height: {}
           Weight: {}
           Age: {}
           AC: {}
           Initiative: {}
           HP: {}
           Temp HP: {}
           Lvl: {}
           XP: {}
           Stats:
           STR: {:#?} | 
           DEX: {:#?} | 
           CON: {:#?} | 
           INT: {:#?} | 
           WIS: {:#?} | 
           CHA: {:#?}
           Saving Throws:
           STR: {:#?} | 
           DEX: {:#?} | 
           CON: {:#?} | 
           INT: {:#?} | 
           WIS: {:#?} | 
           CHA: {:#?}
           ",
            self.id,
            self.name,
            self.alignment,
            self.proficiency_bonus,
            self.passive_perception,
            self.inspiration,
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
            self.stats.get("str"),
            self.stats.get("dex"),
            self.stats.get("con"),
            self.stats.get("int"),
            self.stats.get("wis"),
            self.stats.get("cha"),
            self.saving_throws.get("str"),
            self.saving_throws.get("dex"),
            self.saving_throws.get("con"),
            self.saving_throws.get("int"),
            self.saving_throws.get("wis"),
            self.saving_throws.get("cha"),
        )
    }
}

impl Character {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn test_character_frank() -> Self {
        Character {
            id: None,
            name: "Frank".to_string(),
            alignment: "Neutral".to_string(),
            stats: HashMap::from([
                ("str".to_string(), 12),
                ("dex".to_string(), 12),
                ("con".to_string(), 12),
                ("int".to_string(), 12),
                ("wis".to_string(), 12),
                ("cha".to_string(), 12),
            ]),
            saving_throws: HashMap::from([
                ("str".to_string(), true),
                ("dex".to_string(), false),
                ("con".to_string(), true),
                ("int".to_string(), false),
                ("wis".to_string(), true),
                ("cha".to_string(), false),
            ]),
            proficiency_bonus: 2,
            passive_perception: 12,
            inspiration: false,
            languages: Vec::new(),
            proficiencies: Vec::new(),
            invintory: Vec::new(),
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
            xp: 0,
        }
    }

    pub fn test_character_kevin() -> Self {
        Character {
            id: None,
            name: "Kevin".to_string(),
            alignment: "Evil".to_string(),
            stats: HashMap::from([
                ("str".to_string(), 20),
                ("dex".to_string(), 20),
                ("con".to_string(), 20),
                ("int".to_string(), 20),
                ("wis".to_string(), 20),
                ("cha".to_string(), 20),
            ]),
            saving_throws: HashMap::from([
                ("str".to_string(), false),
                ("dex".to_string(), true),
                ("con".to_string(), false),
                ("int".to_string(), true),
                ("wis".to_string(), false),
                ("cha".to_string(), true),
            ]),
            proficiency_bonus: 2,
            passive_perception: 12,
            inspiration: false,
            languages: Vec::new(),
            proficiencies: Vec::new(),
            invintory: Vec::new(),
            speed: 30,
            gender: "Female".to_string(),
            height: 7,
            weight: 200,
            age: 35,
            armor_class: 23,
            initiative: 4,
            hit_points: 20,
            temp_hit_points: 1,
            level: 1,
            xp: 0,
        }
    }

    pub fn print_character(&self) {
        println!("{}", self);

        for lang in &self.languages {
            println!("{}", lang);
        }

        for prof in &self.proficiencies {
            println!("{}", prof);
        }

        for item in &self.invintory {
            println!("{}", item);
        }
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

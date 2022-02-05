use crate::data::{
    background::Background, class::Class, items::Item, language::Language,
    proficiency::Proficiency, race::Race, spells::Spell, alignments::Alignment,
    stats::Stats, gender::Gender,
    stats::Stats::{STR, DEX, CON, INT, WIS, CHA},
};
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone)]
// TODO: Consider sets instead of Vecs for data structures
// like languages, proficiencies, ect. 
pub struct Character {
    pub id: Option<i64>,
    pub name: String,
    //    pub race: Race,
    //    pub class: Class,
    //    pub background: Background,
    pub alignment: Alignment,
    pub stats: HashMap<Stats, u8>,
    pub proficiencies: Vec<Proficiency>,
    pub saving_throws: HashMap<Stats, bool>,
    // TODO: This value should be calculated automatically 
    // via the experience table 
    pub proficiency_bonus: u8,
    // TODO: Should also be calculated automatically
    // from appropriate stats values
    pub passive_perception: u8,
    pub inspiration: bool,
    pub languages: Vec<Language>,
    pub invintory: Vec<Item>,
    pub spells: Vec<Spell>,
    // TODO: Calculated from Race and/or class 
    pub speed: u8,
    // TODO: Change to Enum
    pub gender: Gender,
    pub height: u8,
    pub weight: u8,
    pub age: u8,
    pub armor_class: u8,
    pub initiative: u8,
    pub hit_points: u16,
    pub temp_hit_points: u16,
    // TODO: The xp value can be adjusted manually by the user,
    // level can auto adjust from xp value via the experience 
    // table.
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
           self.stats.get(&STR),
           self.stats.get(&DEX),
           self.stats.get(&CON),
           self.stats.get(&INT),
           self.stats.get(&WIS),
           self.stats.get(&CHA),
           self.saving_throws.get(&STR),
           self.saving_throws.get(&DEX),
           self.saving_throws.get(&CON),
           self.saving_throws.get(&INT),
           self.saving_throws.get(&WIS),
           self.saving_throws.get(&CHA),
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
            alignment: Alignment::Neutral,
            stats: HashMap::from([
                (STR, 12),
                (DEX, 12),
                (CON, 12),
                (INT, 12),
                (WIS, 12),
                (CHA, 12),
            ]),
            saving_throws: HashMap::from([
                (STR, true),
                (DEX, false),
                (CON, true),
                (INT, false),
                (WIS, true),
                (CHA, false),
            ]),
            proficiency_bonus: 2,
            passive_perception: 12,
            inspiration: false,
            speed: 30,
            gender: Gender::Male,
            height: 6,
            weight: 100,
            age: 30,
            armor_class: 13,
            initiative: 3,
            hit_points: 10,
            temp_hit_points: 0,
            level: 1,
            xp: 0,
            languages: Vec::new(),
            proficiencies: Vec::new(),
            invintory: Vec::new(),
            spells: Vec::new(),
        }
    }

    pub fn test_character_kevin() -> Self {
        Character {
            id: None,
            name: "Kevin".to_string(),
            alignment: Alignment::LawfulGood,
            stats: HashMap::from([
                (STR, 20),
                (DEX, 20),
                (CON, 20),
                (INT, 20),
                (WIS, 20),
                (CHA, 20),
            ]),
            saving_throws: HashMap::from([
                (STR, false),
                (DEX, true),
                (CON, false),
                (INT, true),
                (WIS, false),
                (CHA, true),
            ]),
            proficiency_bonus: 2,
            passive_perception: 12,
            inspiration: false,
            speed: 30,
            gender: Gender::Female,
            height: 7,
            weight: 200,
            age: 35,
            armor_class: 23,
            initiative: 4,
            hit_points: 20,
            temp_hit_points: 1,
            level: 1,
            xp: 0,
            languages: Vec::new(),
            proficiencies: Vec::new(),
            invintory: Vec::new(),
            spells: Vec::new(),
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
        for spell in &self.spells {
            println!("{}", spell);
        }
    }
}

#[derive(Default, Clone)]
// A SavedCharacter is a lightweight character representation
// that holds only the most basic information. This is used
// by the SelectScreen state to display a menu of all currently-saved
// characters in the database, so we don't have to enstantiate
// an entire Character struct for each saved character when
// at the SelectScreen.
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

use crate::data::{
    alignments::Alignment,
    items::Item,
    language::Language,
    proficiency::Proficiency,
    spells::Spell,
    stats::Stats,
    stats::Stats::{CHA, CON, DEX, INT, STR, WIS},
};
use rusqlite::{Result, Row, ToSql};
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone)]
// TODO: Consider sets instead of Vecs for data structures
// like languages, proficiencies, ect.
// TODO: DON'T FORGET FEATS!
pub struct Character {
    pub id: Option<i64>,
    pub name: String,
    //    pub race: Race,
    //    pub class: Class,
    //    pub background: Option<String>,
    //    pub personality_trait: String,
    //    pub ideal: String,
    //    pub bond: String,
    //    pub flaw: String,
    //    pub features: Vec<Feature>,
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
    pub gender: String,
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

pub trait Model: std::fmt::Display {
    fn build(&self, row: &Row) -> Result<()>
    where
        Self: Sized;
    fn parameters(&self) -> Vec<Box<dyn ToSql>>;
    fn table(&self) -> String;
    fn columns(&self) -> String;
    fn queries(&self) -> String;
    fn values(&self) -> String;
}

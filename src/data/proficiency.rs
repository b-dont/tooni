use std::{fmt, str::FromStr};

#[derive(Clone)]
pub enum ProficiencyClass {
    Skill,
    Armor,
    Weapon,
    Tool,
}

impl Default for ProficiencyClass {
    fn default() -> Self {
        ProficiencyClass::Skill
    }
}

impl FromStr for ProficiencyClass {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Skill" => Ok(ProficiencyClass::Skill),
            "Armor" => Ok(ProficiencyClass::Armor),
            "Weapon" => Ok(ProficiencyClass::Weapon),
            "Tool" => Ok(ProficiencyClass::Tool),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ProficiencyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ProficiencyClass::Skill => write!(f, "Skill"),
            &ProficiencyClass::Armor => write!(f, "Armor"),
            &ProficiencyClass::Weapon=> write!(f, "Weapon"),
            &ProficiencyClass::Tool=> write!(f, "Tool"),
        }
    }
}

#[derive(Default, Clone)]
pub struct Proficiency {
    pub id: Option<i64>,
    pub name: String,
    // TODO: Change class to Enum;
    // Skill, Armor, Weapon, Tool, ect.
    pub class: ProficiencyClass,
}

impl fmt::Display for Proficiency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, Name: {}, Class: {}",
            self.id, self.name, self.class
        )
    }
}

impl Proficiency {

    pub fn new(id: i64, name: String, class: ProficiencyClass) -> Self {
        Self {
            id: Some(id),
            name,
            class
        }
    }
}

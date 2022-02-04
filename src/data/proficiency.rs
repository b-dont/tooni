use std::fmt;

#[derive(Clone)]
pub enum ProficiencyClass {
    Skill,
    Armor,
    Weapon,
    Tool,
    None
}

impl Default for ProficiencyClass {
    fn default() -> Self {
        ProficiencyClass::Skill
    }
}

impl fmt::Display for ProficiencyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ProficiencyClass::Skill => write!(f, "Skill"),
            &ProficiencyClass::Armor => write!(f, "Armor"),
            &ProficiencyClass::Weapon=> write!(f, "Weapon"),
            &ProficiencyClass::Tool=> write!(f, "Tool"),
            &ProficiencyClass::None => write!(f, ""),
        }
    }
}

impl ProficiencyClass {
    pub fn get_string(&self) -> String {
        match self {
            &ProficiencyClass::Skill => "Skill".to_string(),
            &ProficiencyClass::Armor => "Armor".to_string(),
            &ProficiencyClass::Weapon => "Weapon".to_string(),
            &ProficiencyClass::Tool => "Tool".to_string(),
            &ProficiencyClass::None => "".to_string()
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

    pub fn new(id: i64, name: String, class: String) -> Self {
        Self {
            id: Some(id),
            name,
            class: {
                match class.as_str() {
                    "Skill" => ProficiencyClass::Skill,
                    "Armor" => ProficiencyClass::Armor,
                    "Weapon" => ProficiencyClass::Weapon,
                    "Tool" => ProficiencyClass::Tool,
                    _ => ProficiencyClass::None
                }
            }
        }
    }
}

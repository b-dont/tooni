use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Proficiency {
    pub id: Option<i64>,
    pub name: String,
    // TODO: Change class to Enum;
    // Skill, Armor, Weapon, Tool, ect.
    pub class: String,
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
    pub fn new() -> Self {
        Self::default()
    }
}

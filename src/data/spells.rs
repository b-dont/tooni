use ::std::fmt;

#[derive(Default, Debug, Clone)]
// Enum for school
pub struct Spell {
    pub id: Option<i64>,
    pub name: String,
    pub school: String,
    pub level: u8,
    pub casting_time: u8,
    pub range: u8,
    pub components: String,
    pub duration: u8,
    pub description: String,
}

impl fmt::Display for Spell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, 
            Name: {}, 
            School: {},
            Level: {},
            Casting Time: {},
            Range: {},
            Components: {:#?},
            Duration: {},
            Description: {}",
            self.id,
            self.name,
            self.school,
            self.level,
            self.casting_time,
            self.range,
            self.components,
            self.duration,
            self.description
        )
    }
}

impl Spell {
    pub fn new() -> Self {
        Self::default()
    }
}

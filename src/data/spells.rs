#[derive(Default, Debug, Clone)]
// Enum for school
pub struct Spell {
    pub id: Option<i64>,
    pub name: String,
    pub school: String,
    pub level: u8,
    pub casting_time: u8,
    pub range: u8,
    pub components: Vec<String>,
    pub duration: u8,
    pub description: String,
}

#[derive(Default)]

pub struct Character {
    pub name: String,
    pub race: String,
    pub class: String,
    pub background: String,
    pub alignment: String,
    pub xp: u8,
    pub id: Option<u8>,
}

impl Character {
    pub fn new() -> Self {
        Self::default()
   }
}

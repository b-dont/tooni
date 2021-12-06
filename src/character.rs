pub struct Character {
    pub name: String,
    pub class: String,
    pub xp: u8,
    pub id: Option<u8>,
}

impl Character {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            class: "".to_string(),
            xp: 0,
            id: None,
        }
    }
}

#[derive(Default)]

// The character struct is used by both the database interface
// and the TUI interface to save and render character data.
// Each of the struct's elements is public and represents a piece of
// character data. The only impl is `new()` which will call `default()`.
//
// TODO: As of right now, there has been no need for additional impls
// on this struct. This may change with additional features.
// The struct does not currently contain all needed elements of data,
// and consideration is needed for possible enums or additional structs
// for data such as race, class, background, and others.

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

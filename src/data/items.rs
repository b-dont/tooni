use::std::fmt;

#[derive(Default, Debug, Clone)]
// Enum for item class.
pub struct Item {
    pub id: Option<i64>,
    pub name: String,
    pub class: String,
    pub quantity: u16,
    pub value: u16,
    pub weight: u64,
    pub properties: String,
    pub description: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, 
            Name: {}, 
            Class: {},
            Quantity: {},
            Value: {},
            Weight: {},
            Properties: {},
            Description: {}",
            self.id, 
            self.name, 
            self.class,
            self.quantity,
            self.value,
            self.weight,
            self.properties,
            self.description
        )
    }
}

impl Item {
    pub fn new() -> Self {
        Self::default()
    }
}

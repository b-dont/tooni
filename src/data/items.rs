#[derive(Default, Debug, Clone)]
// Enum for item class.
pub struct Item {
   pub id: Option<i64>,
   pub name: String,
   pub class: String,
   pub value: u16,
   pub damage: Option<(u8, u8)>,
   pub weight: u64,
   pub properties: String
}

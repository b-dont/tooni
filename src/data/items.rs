#[derive(Default, Debug, Clone)]
pub struct Item {
   pub name: String,
   pub class: String,
   pub cost: u16,
   pub damage: Option<(u8, u8)>,
   pub weight: u16,
   pub properties: String
}

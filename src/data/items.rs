use ::std::{fmt, str::FromStr};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug, Clone)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
    Unknown,
}

impl FromSql for ItemRarity {
    #[inline]
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<ItemRarity> {
        Ok(ItemRarity::from_str(value.as_str()?).unwrap())
    }
}

impl FromStr for ItemRarity {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Common" => Ok(ItemRarity::Common),
            "Uncommon" => Ok(ItemRarity::Uncommon),
            "Rare" => Ok(ItemRarity::Rare),
            "VeryRare" => Ok(ItemRarity::VeryRare),
            "Legendary" => Ok(ItemRarity::Legendary),
            "Unknown" => Ok(ItemRarity::Unknown),
            _ => Err(()),
        }
    }
}

impl fmt::Display for ItemRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ItemRarity::Common => write!(f, "Common"),
            &ItemRarity::Uncommon => write!(f, "Uncommon"),
            &ItemRarity::Rare => write!(f, "Rare"),
            &ItemRarity::VeryRare => write!(f, "VeryRare"),
            &ItemRarity::Legendary => write!(f, "Legendary"),
            &ItemRarity::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Item {
    pub id: Option<i64>,
    pub name: String,
    // TODO: Tried making an Enum for "class",
    // but it seems there are potnentially an
    // unlimited number of item "classes";
    // users can establish their own item class,
    // and the D&D modules themselves have a very
    // large number of "class" or "types" for items.
    // For now, this will remain a String, and the user
    // will be responsible for establishing the "class" of
    // item when one is added.
    pub class: String,
    pub quantity: u16,
    pub rarity: Option<ItemRarity>,
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
            Rarity: {:#?},
            Value: {},
            Weight: {},
            Properties: {},
            Description: {}",
            self.id,
            self.name,
            self.class,
            self.quantity,
            self.rarity,
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

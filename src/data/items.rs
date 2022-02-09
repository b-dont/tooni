use crate::data::character::Model;
use ::std::{fmt, str::FromStr};
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result,
};

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
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<ItemRarity> {
        Ok(ItemRarity::from_str(value.as_str()?).unwrap())
    }
}

impl ToSql for ItemRarity {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
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

impl Model for Item {
    fn parameters(&self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(self.id));
        params.push(Box::new(self.name.clone()));
        params.push(Box::new(self.class.clone()));
        params.push(Box::new(self.quantity));
        params.push(Box::new(self.rarity.clone()));
        params.push(Box::new(self.value));
        params.push(Box::new(self.weight));
        params.push(Box::new(self.properties.clone()));
        params.push(Box::new(self.description.clone()));
        params
    }

    fn build_model(&self) -> Item 
    where Self : Sized {
        Item {
            id: self.id,
            name: self.name.clone(),
            class: self.class.clone(),
            quantity: self.quantity,
            rarity: self.rarity.clone(),
            value: self.value,
            weight: self.weight,
            properties: self.properties.clone(),
            description: self.description.clone()
        }
    }

    fn add_junctions(&self, juncts: Vec<Box<impl Model>>)
    where Self : Sized {
        
    }

    fn new(&self) -> Item {
        Self::default()    
    }
}

impl Item {
    pub fn new() -> Self {
        Self::default()
    }
}

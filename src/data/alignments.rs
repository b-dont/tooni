use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use std::{fmt, str::FromStr};

#[derive(Clone)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    Neutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

impl FromSql for Alignment {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Alignment> {
        Ok(Alignment::from_str(value.as_str()?).unwrap())
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Neutral
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Alignment::LawfulGood => write!(f, "Lawful Good"),
            &Alignment::NeutralGood => write!(f, "Neutral Good"),
            &Alignment::ChaoticGood => write!(f, "Chaotic Good"),
            &Alignment::LawfulNeutral => write!(f, "Lawful Neutral"),
            &Alignment::Neutral => write!(f, "Neutral"),
            &Alignment::ChaoticNeutral => write!(f, "Chaotic Neutral"),
            &Alignment::LawfulEvil => write!(f, "Lawful Neutral"),
            &Alignment::NeutralEvil => write!(f, "Neutral Evil"),
            &Alignment::ChaoticEvil => write!(f, "Chaotic Evil"),
        }
    }
}

impl FromStr for Alignment {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Lawful Good" => Ok(Alignment::LawfulGood),
            "Neutral Good" => Ok(Alignment::NeutralGood),
            "Chaotic Good" => Ok(Alignment::ChaoticGood),
            "Lawful Evil" => Ok(Alignment::LawfulEvil),
            "Neutral Evil" => Ok(Alignment::NeutralEvil),
            "Chaotic Evil" => Ok(Alignment::ChaoticEvil),
            "Lawful Neutral" => Ok(Alignment::LawfulNeutral),
            "Neutral" => Ok(Alignment::Neutral),
            "Chaotic Neutral" => Ok(Alignment::ChaoticNeutral),
            _ => Err(()),
        }
    }
}

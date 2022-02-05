use std::{fmt, str::FromStr};

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Stats {
    STR,
    DEX,
    CON,
    INT,
    WIS,
    CHA,
}

impl Default for Stats {
    fn default() -> Self {
        Stats::STR
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Stats::STR => write!(f, "STR"),
            &Stats::DEX => write!(f, "DEX"),
            &Stats::CON => write!(f, "CON"),
            &Stats::INT => write!(f, "INT"),
            &Stats::WIS => write!(f, "WIS"),
            &Stats::CHA => write!(f, "CHA"),
        }
    }
}

impl FromStr for Stats {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "STR" => Ok(Stats::STR),
            "DEX" => Ok(Stats::DEX),
            "CON" => Ok(Stats::CON),
            "INT" => Ok(Stats::INT),
            "WIS" => Ok(Stats::WIS),
            "CHA" => Ok(Stats::CHA),
            _ => Err(()),
        }
    }
}

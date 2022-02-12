use crate::data::character::Model;
use rusqlite::{Result, Row, ToSql};
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Language {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
}

impl Language {
    pub fn new(&self) -> Self {
        Self::default()
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID: {:#?}, Name: {}, Description: {}",
            self.id, self.name, self.description
        )
    }
}

impl Model for Language {
    fn parameters(&self) -> Vec<Box<dyn ToSql>> {
        let mut params: Vec<Box<dyn ToSql>> = Vec::new();
        params.push(Box::new(self.id));
        params.push(Box::new(self.name.clone()));
        params.push(Box::new(self.description.clone()));
        params
    }

    fn build(&self, row: &Row) -> Result<()>
    where
        Self: Sized,
    {
        self.id = row.get(0)?;
        self.name = row.get(1)?;
        self.description = row.get(2)?;

        Ok(())
    }

    fn table(&self) -> String {
        "languages".to_string()
    }

    fn columns(&self) -> String {
        "id INTEGER PRIMARY KEY,
        name TEXT UNIQUE NOT NULL,
        description TEXT UNIQUE NOT NULL"
            .to_string()
    }

    fn queries(&self) -> String {
        "id, name, description".to_string()
    }

    fn values(&self) -> String {
        "?1, ?2, ?3".to_string()
    }
}

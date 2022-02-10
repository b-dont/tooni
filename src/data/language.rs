use crate::data::character::Model;
use rusqlite::ToSql;
use std::fmt;

#[derive(Default, Debug, Clone)]
pub struct Language {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
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

    fn build_model(&self) -> Language 
    where Self : Sized {
        Language {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone()
        }
    }

    fn add_junctions(&self, juncts: Vec<Box<impl Model>>)
    where Self : Sized {
        
    }
}

impl Language {
    pub fn new() -> Self {
        Self::default()
    }
}

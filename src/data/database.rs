use super::character::Model;
use crate::data::{
    character::SavedCharacter,
    tables::{JunctionTable, Table},
};
use rusqlite::{params, params_from_iter, Connection, Result};

// TODO: Consider PRAGMA SQLite statement at connection open
pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        Ok(Self {
            connection: Connection::open("data.sqlite3")?,
        })
    }

    pub fn create_junction_table(&self, junct: JunctionTable) -> Result<()> {
        self.connection.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                    {} INTEGER REFERENCES {}(id), 
                    {} INTEGER REFERENCES {}(id), 
                    PRIMARY KEY ({}, {}))",
                junct.name(),
                junct.columns().0,
                junct.references().0,
                junct.columns().1,
                junct.references().1,
                junct.columns().0,
                junct.columns().1,
            )
            .as_str(),
            [],
        )?;

        Ok(())
    }

    pub fn load(&self, id: i64, model: &impl Model) -> Result<()> {
        let mut stmt = self.connection.prepare(
            format!(
                "SELECT {} FROM {} WHERE id=?1",
                model.queries(),
                model.table()
            )
            .as_str(),
        )?;

        stmt.query_row(params![id], |row| Ok(model.build(&row)))?;
        Ok(())
    }

    pub fn save(&self, model: &impl Model) -> Result<()> {
        let mut table_stmt = self.connection.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                model.table(),
                model.columns()
            )
            .as_str(),
            [],
        )?;

        let mut stmt = self.connection.prepare(
            format!(
                "REPLACE INTO {} ({}) VALUES ({})",
                model.table(),
                model.queries(),
                model.values()
            )
            .as_str(),
        )?;

        stmt.execute(params_from_iter(model.parameters().into_iter()))?;
        Ok(())
    }

    pub fn load_junction(&self, junct: JunctionTable, id: i64) -> Result<Vec<Box<impl Model>>> {
        let mut stmt = self.connection.prepare(
            format!(
                "SELECT {}, {} FROM {} WHERE {}=?1",
                junct.columns().0,
                junct.columns().1,
                junct.name(),
                junct.columns().0
            )
            .as_str(),
        )?;

        let queried_models =
            stmt.query_map(params![id], |row| Ok(self.load(row.get(0)?, row.get(1)?)?))?;
        queried_models.into_iter().collect()
    }

    pub fn save_junction(&self, model: &impl Model) -> Result<()> {
        let mut stmt = self.connection.prepare(
            format!(
                "REPLACE INTO {} ({}, {}) VALUES ({})",
                junct.name(),
                junct.columns().0,
                junct.columns().1,
                junct.values()
            )
            .as_str(),
        )?;

        stmt.execute(params![object, source])?;
        Ok(())
    }

    pub fn get_all_models(&self, model: &impl Model) -> Result<Vec<Box<impl Model>>> {
        let mut stmt = self
            .connection
            .prepare(format!("SELECT {} FROM {}", model.queries(), model.table()).as_str())?;

        let rows = stmt.query_map([], |row| Ok(Box::new(model.build(&row)?.clone())))?;
        let mut all_models: Vec<Box<_>> = vec![];

        for result in rows {
            all_models.push(result?);
        }

        Ok(all_models)

    }

    pub fn delete_row(&self, id: i64, table: Table) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare(format!("DELETE FROM {} WHERE id=?1", table.name()).as_str())?;

        stmt.execute(params![id])?;
        Ok(())
    }

    //    TODO: This method here will need to change, given the other db method and character struct
    //    changes.
    //    pub fn list_all_characters(&self) -> Result<Vec<SavedCharacter>> {
    //        let mut stmt = self
    //            .connection
    //            .prepare("SELECT id, name, race, class FROM characters")?;
    //        let characters = stmt.query_map([], |row| {
    //            Ok(SavedCharacter {
    //                id: row.get(0)?,
    //                name: row.get(1)?,
    //                race: row.get(2)?,
    //                class: row.get(3)?,
    //            })
    //        })?;
    //        characters.into_iter().collect()
    //    }
}

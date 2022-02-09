use super::character::Model;
use crate::data::{
    character::SavedCharacter,
    tables::{JunctionTable, Table},
};
use enum_iterator::IntoEnumIterator;
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

    pub fn create_all_tables(&self) -> Result<()> {
        for table in Table::into_enum_iter() {
            self.create_table(table)?;
        }
        for junct in JunctionTable::into_enum_iter() {
            self.create_junction_table(junct)?;
        }

        Ok(())
    }

    pub fn create_table(&self, table: Table) -> Result<()> {
        self.connection.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                table.name(),
                table.columns()
            )
            .as_str(),
            [],
        )?;

        Ok(())
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

    pub fn save_to_junction(&self, junct: JunctionTable, object: i64, source: i64) -> Result<()> {
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

    pub fn load_from_junction(&self, junct: JunctionTable, id: i64) -> Result<Vec<Box<impl Model>>> {
        let mut stmt = self.connection.prepare(
            format!("SELECT {}, {} FROM {} WHERE {}=?1",
                    junct.columns().0,
                    junct.columns().1,
                    junct.name(),
                    junct.columns().0
                ).as_str())?;

        let queried_models = stmt.query_map(params![id], |row|{
            Ok(self.load(row.get(0)?, row.get(1)?)?)
        })?;
        queried_models.into_iter().collect()
    }

    pub fn delete_row(&self, id: i64, table: Table) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare(format!("DELETE FROM {} WHERE id=?1", table.name()).as_str())?;

        stmt.execute(params![id])?;
        Ok(())
    }

    pub fn save(&self, table: Table, model: &impl Model) -> Result<()> {
        let mut stmt = self.connection.prepare(
            format!(
                "REPLACE INTO {} ({}) VALUES ({})",
                table.name(),
                table.queries(),
                table.values()
            )
            .as_str(),
        )?;

        stmt.execute(params_from_iter(model.parameters().into_iter()))?;
        Ok(())
    }

    pub fn load(&self, id: i64, table: Table) -> Result<Box<impl Model>> {
        let mut stmt = self.connection.prepare(
            format!(
                "SELECT {} FROM {} WHERE id=?1",
                table.queries(),
                table.name()
            )
            .as_str(),
        )?;

        let queried_model = stmt.query_row(params![id], |row| Ok(table.create_model(&row)))?;

        if table.has_junctions() {
            for junct in table.junctions().unwrap() {
                let new_junct = self.load_from_junction(junct, id)?;
                queried_model.as_ref().unwrap().add_junctions(new_junct);
            }
        }
        
        queried_model
    }

    pub fn get_all_rows(&self, table: Table) -> Result<Vec<Box<impl Model>>> {
        let mut stmt = self
            .connection
            .prepare(format!("SELECT {} FROM {}", table.queries(), table.name()).as_str())?;

        let all_models = stmt.query_map([], |row| Ok(table.create_model(&row)?))?;
        all_models.into_iter().collect()
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

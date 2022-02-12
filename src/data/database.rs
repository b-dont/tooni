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

    pub fn load<T: Model>(&self, id: i64) -> Result<T> {
        let mut stmt = self.connection.prepare(
            format!(
                "SELECT {} FROM {} WHERE id=?1",
                T::queries(),
                T::table()
            )
            .as_str(),
        )?;

        stmt.query_row(params![id], |row| Ok(T::build(&row)))?
    }

    pub fn save<T: Model>(&self, model: T) -> Result<()> {
        let mut table_stmt = self.connection.execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                T::table(),
                T::columns()
            )
            .as_str(),
            [],
        )?;

        let mut stmt = self.connection.prepare(
            format!(
                "REPLACE INTO {} ({}) VALUES ({})",
                T::table(),
                T::queries(),
                T::values()
            )
            .as_str(),
        )?;

        stmt.execute(params_from_iter(model.parameters().into_iter()))?;
        Ok(())
    }

    pub fn save_junction<T: Model>(&self, model: T) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare(
                format!(
                    "REPLACE INTO {} ({}) VALUES (?1, ?2)"
                    ,
                    ).as_str()
            )?;
    }

    pub fn get_all_models<T: Model>(&self) -> Result<Vec<T>> {
        let mut stmt = self
            .connection
            .prepare(format!("SELECT {} FROM {}", T::queries(), T::table()).as_str())?;

        let rows = stmt.query_map([], |row| Ok(T::build(&row)?))?;
        rows.into_iter().collect()
    }

    pub fn delete<T: Model>(&self, id: i64) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare(format!("DELETE FROM {} WHERE id=?1", T::table()).as_str())?;

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

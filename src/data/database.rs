use super::character::Model;
use crate::data::character::SavedCharacter;
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
        let mut stmt = self
            .connection
            .prepare(format!("SELECT {} FROM {} WHERE id=?1", T::queries(), T::table()).as_str())?;

        let mut loaded_model = stmt
            .query_row(params![id], |row| Ok(T::build(&row)))?
            .unwrap();

        if T::has_junctions() {
            for table in T::junct_tables().unwrap_or(vec![]) {
                let junction_ids =
                    self.load_junction_ids(T::junct_columns(&table).unwrap(), &table, id)?;
                for junct_id in junction_ids {
                    self.load_junction(
                        &table,
                        &T::junct_columns(&table).unwrap().1,
                        &T::junct_qeries(&table).unwrap(),
                        junct_id,
                        &mut loaded_model,
                    )?;
                }
            }
        }
        Ok(loaded_model)
    }

    pub fn save<T: Model>(&self, model: &T) -> Result<()> {
        self.connection.execute(
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

        if T::has_junctions() {
            self.save_junctions(model);
        }

        Ok(())
    }

    pub fn delete<T: Model>(&self, id: i64) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare(format!("DELETE FROM {} WHERE id=?1", T::table()).as_str())?;

        stmt.execute(params![id])?;
        Ok(())
    }

    pub fn save_junctions<T: Model>(&self, model: &T) -> Result<()> {
        for table in T::junct_tables().unwrap_or(vec![]) {
            self.connection.execute(
                format!(
                    "CREATE TABLE IF NOT EXISTS {} (
                        {} INTEGER REFERENCES {}(id),
                        {} INTEGER REFERENCES {}(id),
                        PRIMARY KEY ({}, {})
                    )",
                    table,
                    T::junct_columns(&table).unwrap().0,
                    T::junct_references(&table).unwrap().0,
                    T::junct_columns(&table).unwrap().1,
                    T::junct_references(&table).unwrap().1,
                    T::junct_columns(&table).unwrap().0,
                    T::junct_columns(&table).unwrap().1
                )
                .as_str(),
                [],
            )?;

            for junct in model.junction_ids(&table).unwrap() {
                self.connection.execute(
                    format!(
                        "REPLACE INTO {} ({}, {}) VALUES (?1, ?2)",
                        table,
                        T::junct_columns(&table).unwrap().0,
                        T::junct_columns(&table).unwrap().1,
                    )
                    .as_str(),
                    [model.id(), Some(junct)],
                )?;
            }
        }
        Ok(())
    }

    pub fn load_junction_ids(
        &self,
        columns: (String, String),
        table: &str,
        id: i64,
    ) -> Result<Vec<i64>> {
        let mut stmt = self.connection.prepare(
            format!(
                "SELECT {}, {} FROM {} WHERE id=?1",
                columns.0, columns.1, table,
            )
            .as_str(),
        )?;

        let ids = stmt.query_map([id], |row| Ok(row.get(1)?))?;
        ids.into_iter().collect()
    }

    pub fn load_junction<T: Model>(
        &self,
        junct_table: &str,
        table: &str,
        queries: &str,
        id: i64,
        model: &mut T,
    ) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare(format!("SELECT {} FROM {} WHERE id=?1", queries, table).as_str())?;
        stmt.query_row([id], |row| Ok(model.build_junction(&junct_table, &row)?))?;

        Ok(())
    }

    pub fn get_all_models<T: Model>(&self) -> Result<Vec<T>> {
        let mut stmt = self
            .connection
            .prepare(format!("SELECT {} FROM {}", T::queries(), T::table()).as_str())?;

        let rows = stmt.query_map([], |row| Ok(T::build(&row)?))?;
        rows.into_iter().collect()
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

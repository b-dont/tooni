use crate::character::{Character, SavedCharacter};
use crossterm::ExecutableCommand;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, Result};

// Database interface.
// This struct and its impls represent
// all needed interaction with the SQLite database.
pub struct Database {
    path: String,
    connection: OnceCell<Connection>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            path: "characters.sqlite3".to_string(),
            connection: OnceCell::new(),
        }
    }

    pub fn get_connection(&self) -> Result<&Connection> {
        self.connection
            .get_or_try_init(|| Connection::open(&self.path))
    }

    // Create a character table in the SQLite database.
    // Each column represents an element of the character sheet.
    pub fn create_character_table(&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY
                name TEXT NOT NULL,
                race TEXT NOT NULL,
                class TEXT NOT NULL,
                background TEXT NOT NULL,
                alignment TEXT NOT NULL,
                xp INTEGER,
            )",
            [],
        )?;

        Ok(())
    }

    // Saves a Character struct to the database. Each of the struct's
    // elements represents a column in the SQLite database.
    // When a character struct is saved with a "None" value for its
    // id element, the database will automatically assign this value as
    // n + 1, where n = the highest id that exists in the database.
    pub fn save_character(&self, character: &Character) -> Result<()> {
        let mut stm = self
            .get_connection()?
            .prepare(
            "REPLACE INTO characters (id, name, race, class, background, alignment, xp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;

        stm.execute([
            character.id,
            character.name,
            character.race,
            character.class,
            character.background,
            character.alignment,
            character.xp
        ]);

        Ok(())
    }

    // Loads a character row from the database that matches the given id.
    // TODO: I think this code will panic if an invalid ID is given;
    // will need to handle this error instead of panicing. This kind of
    // error shouldn't happen, as a user will never call this function with
    // any kind of "custom" id argument.
    pub fn load_character(&self, id: u8) -> Result<Character> {
        let mut stmt = self
            .get_connection()?
            .prepare("SELECT * FROM characters WHERE id=?1")?;

        let queried_character = stmt.query_row(params![id], |row| {
            Ok(Character {
                id: row.get(0)?,
                name: row.get(1)?,
                race: row.get(2)?,
                class: row.get(3)?,
                background: row.get(4)?,
                alignment: row.get(5)?,
                xp: row.get(6)?,
            })
        })?;

        Ok(queried_character)
    }

    // Deletes a SQLite row that matches the id element of the Character
    // struct argument.
    pub fn delete_character(&self, character: &Character) -> Result<()> {
        let mut stmt = self
            .get_connection()?
            .prepare("DELETE FROM characters WHERE id=?1")?;

        stmt.execute([character.id]);

        Ok(())
    }

    // Queries the SQLite DB and creates an iterator of all rows in the DB,
    // we then instantiate a Character struct with each row of data and push it
    // to a vector, which is returned.
    // This is used only for the `select_screen()` function to display all
    // currently saved characters in the database.
    pub fn get_all_characters(&self) -> Result<Vec<Character>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT * FROM characters")?;
        let characters = stmt.query_map([], |row| {
            Ok(Character {
                id: row.get(0)?,
                name: row.get(1)?,
                race: row.get(2)?,
                class: row.get(3)?,
                background: row.get(4)?,
                alignment: row.get(5)?,
                xp: row.get(6)?,
            })
        })?;
        characters.into_iter().collect()
    }

    pub fn list_all_characters(&self) -> Result<Vec<SavedCharacter>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT id, name, race, class FROM characters")?;
        let characters = stmt.query_map([], |row| {
            Ok(SavedCharacter {
                id: row.get(0)?,
                name: row.get(1)?,
                race: row.get(2)?,
                class: row.get(3)?,
            })
        })?;
        characters.into_iter().collect()
    }
}

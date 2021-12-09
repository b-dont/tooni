use crate::character::Character;
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
    // Each column represesnts an element of the character sheet.
    pub fn create_character_table(&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS characters (
                name TEXT NOT NULL,
                race TEXT NOT NULL,
                class TEXT NOT NULL,
                background TEXT NOT NULL,
                alignment TEXT NOT NULL,
                xp INTEGER,
                id INTEGER PRIMARY KEY
            )",
            [],
        )?;

        Ok(())
    }

    // Saves a Character struct to the database. Each of the struct's
    // elemnts represents a column in the SQLite database.
    // When a character struct is saved with a "None" value for its
    // id element, the database will automatically assign this value as
    // n + 1, where n = the highest id that exists in the database.
    pub fn save_character(&self, character: &Character) -> Result<()> {
        self.get_connection()?.execute(
            "REPLACE INTO characters (name, race, class, background, alignment, xp, id)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                character.name,
                character.race,
                character.class,
                character.background,
                character.alignment,
                character.xp,
                character.id
            ],
        )?;

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
                name: row.get(0)?,
                race: row.get(1)?,
                class: row.get(2)?,
                background: row.get(3)?,
                alignment: row.get(4)?,
                xp: row.get(5)?,
                id: row.get(6)?,
            })
        })?;

        Ok(queried_character)
    }

    // Deletes a SQLite row that matches the id element of the Character
    // struct argument.
    pub fn delete_character(&self, character: &Character) -> Result<()> {
        self.get_connection()?
            .execute("DELETE FROM characters WHERE id=?1", params![character.id])?;

        Ok(())
    }

    // Quiries the SQLite DB and creates an iterator of all rows in the DB,
    // we then instantiate a Character struct with each row of data and push it
    // to a vector, which is returned.
    // This is used only for the `select_screen()` function to display all 
    // currently saved characters in the database.
    pub fn get_all_characters(&self) -> Result<Vec<Character>> {
        let conn = self.get_connection()?;
        let mut stmt = conn
            .prepare("SELECT name, race, class, background, alignment, xp, id FROM characters")?;
        let character_iter = stmt.query_map([], |row| {
            Ok(Character {
                name: row.get(0)?,
                race: row.get(1)?,
                class: row.get(2)?,
                background: row.get(3)?,
                alignment: row.get(4)?,
                xp: row.get(5)?,
                id: row.get(6)?,
            })
        })?;

        let mut all_characters = Vec::new();
        for character in character_iter {
            all_characters.push(character.unwrap());
        }

        Ok(all_characters)
    }
}

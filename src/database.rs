use crate::character::Character;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, Result};

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

    pub fn delete_character(&self, character: &Character) -> Result<()> {
        self.get_connection()?
            .execute("DELETE FROM characters WHERE id=?1", params![character.id])?;

        Ok(())
    }

    pub fn get_all_characters(&self) -> Result<Vec<Character>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT name, race, class, background, alignment, xp, id FROM characters")?;
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

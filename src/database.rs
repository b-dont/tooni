use crate::character::Character;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, Result};

pub struct Database {
    path: String,
    connection: OnceCell<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            connection: OnceCell::new(),
        }
    }

    pub fn get_connection(&self) -> Result<&Connection> {
        self.connection
            .get_or_try_init(|| Connection::open(&self.path))
    }

    pub fn create_database(&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS characters (
                name TEXT NOT NULL,
                class TEXT NOT NULL,
                xp INTEGER,
                id INTEGER PRIMARY KEY
            )",
            [],
        )?;

        Ok(())
    }

    pub fn save_character(&self, character: &Character) -> Result<()> {
        self.get_connection()?.execute(
            "REPLACE INTO characters (name, class, xp, id)
            VALUES (?1, ?2, ?3, ?4)",
            params![character.name, character.class, character.xp, character.id],
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
                class: row.get(1)?,
                xp: row.get(2)?,
                id: row.get(3)?,
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
        let mut stmt = conn.prepare("SELECT name, class, xp, id FROM characters")?;
        let character_iter = stmt.query_map([], |row| {
            Ok(Character {
                name: row.get(0)?,
                class: row.get(1)?,
                xp: row.get(2)?,
                id: row.get(3)?,
            })
        })?;

        let mut all_characters = Vec::new();
        for character in character_iter {
            all_characters.push(character.unwrap());
        }

        Ok(all_characters)
    }
}

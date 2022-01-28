use crate::{data::character::SavedCharacter, Character};
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
            path: "data.sqlite3".to_string(),
            connection: OnceCell::new(),
        }
    }

    pub fn get_connection(&self) -> Result<&Connection> {
        self.connection
            .get_or_try_init(|| Connection::open(&self.path))
    }

    pub fn create_tables(&self) -> Result<()> {
        self.create_character_table();
        self.create_spells_table();
        self.create_items_table();
        self.create_backgrounds_table();
        self.create_classes_table();
        self.create_races_table();

        Ok(())
    }

    // Create a character table in the SQLite database.
    // Each column represents an element of the character sheet.
    pub fn create_character_table(&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                raceconfigid INTEGER,
                FOREIGN KEY(raceconfigid) REFERENCES raceconfigs(id),
                classconfigid INTEGER,
                FOREIGN KEY(classconfigid) REFERENCES classconfigs(id),
                backgroundconfigid INTEGER,
                FOREIGN KEY(backgroundconfigid) REFERENCES backgroundconfigs(id),
                alignment TEXT NOT NULL,
                stats INTEGER,
                FOREIGN KEY(stats) REFERENCES statsconfigs(id),

                proficiencies !TODO,

                proficiency_bonus INTEGER,

                languages !TODO,
                equipment !TODO,
                spells !TODO,

                speed INTEGER,
                gender TEXT NOT NULL,
                height INTEGER,
                weight INTEGER,
                age INTEGER,
                level INTEGER,
                xp INTEGER,
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_stats_table(&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS statsconfigs (
                id INTEGER PRIMARY KEY,
                str INTEGER,
                dex INTEGER,
                con INTEGER,
                int INTEGER,
                wis INTEGER,
                cha INTEGER
            )", 
            []
        )?;

        Ok(())
    }

    pub fn create_spells_table (&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS spells (
                name TEXT NOT NULL PRIMARY KEY,
                school TEXT NOT NULL,
                level INTEGER,
                casting_time INTEGER,
                range INTEGER,
                components TEXT NOT NULL,
                duration INTEGER,
                description TEXT NOT NULL
            )", 
            []
        )?;
        Ok(())
    }

    pub fn create_items_table (&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS items (
                name TEXT NOT NULL PRIMARY KEY,
                class TEXT NOT NULL,
                cost INTEGER,
                damage INTEGER,
                weight INTEGER,
                properties TEXT NOT NULL
            )", 
            []
        )?;
        Ok(())
    }

    pub fn create_backgrounds_table (&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS backgrounds (
                name TEXT NOT NULL PRIMARY KEY,
                skill_prof TEXT NOT NULL,
                languages TEXT NOT NULL,
                starting_equipment TEXT NOT NULL,
                features TEXT NOT NULL,
                personality_trait TEXT NOT NULL,
                ideal TEXT NOT NULL,
                bond TEXT NOT NULL,
                flaw TEXT NOT NULL
            )", 
            []
        )?;

        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS backgroundconfigs (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                skill_prof TEXT NOT NULL,
                languages TEXT NOT NULL,
                starting_equipment TEXT NOT NULL,
                features TEXT NOT NULL,
                personality_trait TEXT NOT NULL,
                ideal TEXT NOT NULL,
                bond TEXT NOT NULL,
                flaw TEXT NOT NULL
            )", 
            []
        )?;

        Ok(())
    }

    pub fn create_classes_table(&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS classes (
                name TEXT NOT NULL PRIMARY KEY,
                features TEXT NOT NULL,
                skill_prof TEXT NOT NULL,
                armor_prof TEXT NOT NULL,
                weapon_prof TEXT NOT NULL,
                tool_prof TEXT NOT NULL,
                saving_throws TEXT NOT NULL,
                hit_dice INTEGER,
                spells_known INTEGER,
                spell_slots INTEGER,
                spell_slot_level INTEGER
            )", 
            []
        )?;

        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS classconfigs (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                features TEXT NOT NULL,
                skill_prof TEXT NOT NULL,
                armor_prof TEXT NOT NULL,
                weapon_prof TEXT NOT NULL,
                tool_prof TEXT NOT NULL,
                saving_throws TEXT NOT NULL,
                hit_dice INTEGER,
                spells_known INTEGER,
                spell_slots INTEGER,
                spell_slot_level INTEGER
            )", 
            []
        )?;

        Ok(())
    }

    pub fn create_races_table(&self) -> Result<()> {
        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS races (
                name TEXT NOT NULL PRIMARY KEY,
                skill_prof TEXT NOT NULL,
                armor_prof TEXT NOT NULL,
                weapon_prof TEXT NOT NULL,
                features TEXT NOT NULL
            )", 
            []
        )?;

        self.get_connection()?.execute(
            "CREATE TABLE IF NOT EXISTS raceconfigs (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                skill_prof TEXT NOT NULL,
                armor_prof TEXT NOT NULL,
                weapon_prof TEXT NOT NULL,
                features TEXT NOT NULL
            )", 
            []
        )?;

        Ok(())
    }

    // Saves a Character struct to the database. Each of the struct's
    // elements represents a column in the SQLite database.
    // When a character struct is saved with a "None" value for its
    // id element, the database will automatically assign this value as
    // n + 1, where n = the highest id that exists in the database.
    pub fn save_character(&self, character: &Character) -> Result<()> {
        let mut stm = self.get_connection()?.prepare(
            "REPLACE INTO characters (id, name, race, class, background, alignment, xp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )?;

        stm.execute(params![
            character.id,
            character.name,
            character.race,
            character.class,
            character.background,
            character.alignment,
            character.xp
        ])?;

        Ok(())
    }

    // Loads a character row from the database that matches the given id.
    // TODO: I think this code will panic if an invalid ID is given;
    // will need to handle this error instead of panicing. This kind of
    // error shouldn't happen, as a user will never call this function with
    // any kind of "custom" id argument.
    pub fn load_character(&self, id: u64) -> Result<Character> {
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

        stmt.execute([character.id])?;

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

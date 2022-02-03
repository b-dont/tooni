use crate::{data::character::SavedCharacter, Character};
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

use super::{language::Language, proficiency::Proficiency, items::Item};

// Database interface.
// This struct and its impls represent
// all needed interaction with the SQLite database.
//
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

    pub fn create_tables(&self) -> Result<()> {
        self.create_character_table()?;
        self.create_languages_tables()?;
        self.create_proficiencies_tables()?;

        Ok(())
    }

    // Create a character table in the SQLite database.
    // Each column represents an element of the character sheet.
    pub fn create_character_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                alignment TEXT NOT NULL,
                proficiency_bonus INTEGER,
                passive_perception INTEGER,
                inspiration INTEGER,
                speed INTEGER,
                gender TEXT NOT NULL,
                height INTEGER,
                weight INTEGER,
                age INTEGER,
                armor_class INTEGER,
                initiative INTEGER,
                hit_points INTEGER,
                temp_hit_points INTEGER,
                level INTEGER,
                xp INTEGER,
                str INTEGER,
                dex INTEGER,
                con INTEGER,
                int INTEGER,
                wis INTEGER,
                cha INTEGER,
                str_saving_throw,
                dex_saving_throw,
                con_saving_throw,
                int_saving_throw,
                wis_saving_throw,
                cha_saving_throw
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_item_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                class TEXT NOT NULL,
                quantity INTEGER,
                value INTEGER,
                weight INTEGER,
                properties TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS character_invintory (
                character INTEGER REFERENCES characters(id),
                item INTEGER REFERENCES items(id),
                PRIMARY KEY (character, item)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn save_character_invintory(
        &self,
        id: Option<i64>,
        items: &Vec<Item>,
    ) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO character_invintory (
                character,
                item 
            )
            VALUES (?1, ?2)",
        )?;

        for item in items {
            stmt.execute(params![id, item.id])?;
        }
        Ok(())
    }

    pub fn save_item(&self, item: Item) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO items (
            id,
            name,
            class,
            quantity,
            value,
            weight,
            properties,
            description
            )
            VALUES (
                ?1, 
                ?2, 
                ?3, 
                ?4, 
                ?5 
                ?6
                ?7
                ?8
                )",
        )?;

        stmt.execute(params![
            item.id, 
            item.name, 
            item.class,
            item.quantity,
            item.value,
            item.weight,
            item.properties,
            item.description
        ])?;
        Ok(())
    }

    pub fn load_character_invintory(&self, id: i64) -> Result<Vec<Item>> {
        let mut stmt = self.connection.prepare(
            "SELECT
            character,
            item
            FROM character_invintory WHERE character=?1
            ",
        )?;

        let invintory =
            stmt.query_map([id], |row| self.load_item(row.get(1)?))?;

        invintory.into_iter().collect()
    }

    pub fn load_item(&self, id: i64) -> Result<Item> {
        let mut stmt = self.connection.prepare(
            "
               SELECT
               id,
               name,
               class,
               quantity,
               value,
               weight,
               properties,
               description
               FROM items WHERE id=?1
            ",
        )?;

        let queried_item = stmt.query_row(params![id], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                quantity: row.get(3)?,
                value: row.get(4)?,
                weight: row.get(5)?, 
                properties: row.get(6)?,
                description: row.get(7)?,
            })
        })?;

        Ok(queried_item)
    }

    pub fn get_all_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self.connection.prepare("SELECT * FROM items")?;

        let items = stmt.query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
                quantity: row.get(3)?,
                value: row.get(4)?,
                weight: row.get(5)?, 
                properties: row.get(6)?,
                description: row.get(7)?,
            })
        })?;
        items.into_iter().collect()
    }


    pub fn create_proficiencies_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS proficiencies (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                class TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS character_proficiencies (
                character INTEGER REFERENCES characters(id),
                proficiency INTEGER REFERENCES proficiencies(id),
                PRIMARY KEY (character, proficiency)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn save_character_proficiencies(
        &self,
        id: Option<i64>,
        profs: &Vec<Proficiency>,
    ) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO character_proficiencies (
                character,
                proficiency
            )
            VALUES (?1, ?2)",
        )?;

        for prof in profs {
            stmt.execute(params![id, prof.id])?;
        }
        Ok(())
    }

    pub fn save_proficiency(&self, prof: Proficiency) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO proficiencies (
            id,
            name,
            class 
            )
            VALUES (?1, ?2, ?3)",
        )?;

        stmt.execute(params![prof.id, prof.name, prof.class])?;
        Ok(())
    }

    pub fn load_characer_proficiencies(&self, id: i64) -> Result<Vec<Proficiency>> {
        let mut stmt = self.connection.prepare(
            "SELECT
            character,
            proficiency 
            FROM character_proficiencies WHERE character=?1
            ",
        )?;

        let character_proficiencies =
            stmt.query_map([id], |row| self.load_proficiency(row.get(1)?))?;

        character_proficiencies.into_iter().collect()
    }

    pub fn load_proficiency(&self, id: i64) -> Result<Proficiency> {
        let mut stmt = self.connection.prepare(
            "
               SELECT
               id,
               name,
               class
               FROM proficiencies WHERE id=?1
            ",
        )?;

        let queried_prof = stmt.query_row(params![id], |row| {
            Ok(Proficiency {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
            })
        })?;

        Ok(queried_prof)
    }

    pub fn get_all_proficiencies(&self) -> Result<Vec<Proficiency>> {
        let mut stmt = self.connection.prepare("SELECT * FROM proficiencies")?;

        let profs = stmt.query_map([], |row| {
            Ok(Proficiency {
                id: row.get(0)?,
                name: row.get(1)?,
                class: row.get(2)?,
            })
        })?;
        profs.into_iter().collect()
    }

    pub fn create_languages_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS languages (
                id INTEGER PRIMARY KEY, 
                name TEXT UNIQUE NOT NULL,
                description TEXT UNIQUE NOT NULL)",
            [],
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS character_languages (
                character INTEGER REFERENCES characters(id),
                language INTEGER REFERENCES languages(id),
                PRIMARY KEY (character, language)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn save_character_languages(&self, id: Option<i64>, langs: &Vec<Language>) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO character_languages (
                character,
                language
            )
            VALUES (?1, ?2)",
        )?;

        for lang in langs {
            stmt.execute(params![id, lang.id])?;
        }
        Ok(())
    }

    pub fn save_language(&self, lang: Language) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO languages (
            id,
            name,
            description
            )
            VALUES (?1, ?2, ?3)",
        )?;

        stmt.execute(params![lang.id, lang.name, lang.description])?;
        Ok(())
    }

    pub fn load_characer_languages(&self, id: i64) -> Result<Vec<Language>> {
        let mut stmt = self.connection.prepare(
            "SELECT
            character,
            language
            FROM character_languages WHERE character=?1
            ",
        )?;

        let character_languages = stmt.query_map([id], |row| 
            self.load_language(row.get(1)?))?;

        character_languages.into_iter().collect()
    }

    pub fn load_language(&self, id: i64) -> Result<Language> {
        let mut stmt = self.connection.prepare(
            "
               SELECT
               id,
               name,
               description
               FROM languages WHERE id=?1
            ",
        )?;

        let queried_lang = stmt.query_row(params![id], |row| {
            Ok(Language {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?;

        Ok(queried_lang)
    }

    pub fn get_all_languages(&self) -> Result<Vec<Language>> {
        let mut stmt = self.connection.prepare("SELECT * FROM languages")?;

        let languages = stmt.query_map([], |row| {
            Ok(Language {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })?;
        languages.into_iter().collect()
    }

    // Saves a Character struct to the database. Each of the struct's
    // elements represents a column in the SQLite database.
    // When a character struct is saved with a "None" value for its
    // id element, the database will automatically assign this value as
    // n + 1, where n = the highest id that exists in the database.
    pub fn save_character(&self, character: &Character) -> Result<()> {
        let mut stmt = self.connection.prepare(
            // Change to SQLite UPDATE statement
            "REPLACE INTO characters (
            id, 
            name, 
            alignment, 
            proficiency_bonus, 
            passive_perception, 
            inspiration, 
            speed, 
            gender, 
            height, 
            weight, 
            age, 
            armor_class, 
            initiative, 
            hit_points, 
            temp_hit_points, 
            level, 
            xp,
            str,
            dex,
            con,
            int,
            wis,
            cha,
            str_saving_throw,
            dex_saving_throw,
            con_saving_throw,
            int_saving_throw,
            wis_saving_throw,
            cha_saving_throw
            )
            VALUES (
                ?1, 
                ?2, 
                ?3, 
                ?4, 
                ?5, 
                ?6, 
                ?7, 
                ?8, 
                ?9, 
                ?10, 
                ?11, 
                ?12, 
                ?13, 
                ?14, 
                ?15, 
                ?16, 
                ?17, 
                ?18, 
                ?19, 
                ?20, 
                ?21, 
                ?22, 
                ?23,
                ?24, 
                ?25, 
                ?26, 
                ?27, 
                ?28, 
                ?29
                    )",
        )?;

        stmt.execute(params![
            character.id,
            character.name,
            character.alignment,
            character.proficiency_bonus,
            character.passive_perception,
            character.inspiration,
            character.speed,
            character.gender,
            character.height,
            character.weight,
            character.age,
            character.armor_class,
            character.initiative,
            character.hit_points,
            character.temp_hit_points,
            character.level,
            character.xp,
            character.stats.get("str"),
            character.stats.get("dex"),
            character.stats.get("con"),
            character.stats.get("int"),
            character.stats.get("wis"),
            character.stats.get("cha"),
            character.saving_throws.get("str"),
            character.saving_throws.get("dex"),
            character.saving_throws.get("con"),
            character.saving_throws.get("int"),
            character.saving_throws.get("wis"),
            character.saving_throws.get("cha")
        ])?;

        self.save_character_languages(character.id, &character.languages)?;
        self.save_character_proficiencies(character.id, &character.proficiencies)?;
        self.save_character_invintory(character.id, &character.invintory)?;

        Ok(())
    }

    // Loads a character row from the database that matches the given id.
    // TODO: I think this code will panic if an invalid ID is given;
    // will need to handle this error instead of panicing. This kind of
    // error shouldn't happen, as a user will never call this function with
    // any kind of "custom" id argument.
    pub fn load_character(&self, id: i64) -> Result<Character> {
        let mut stmt = self.connection.prepare(
            "SELECT 
                id,
                name,
                alignment,
                proficiency_bonus,
                passive_perception,
                inspiration,
                speed,
                gender,
                height,
                weight,
                age,
                armor_class,
                initiative,
                hit_points,
                temp_hit_points,
                level,
                xp,
                str,
                dex,
                con,
                int,
                wis,
                cha,
                str_saving_throw,
                dex_saving_throw,
                con_saving_throw,
                int_saving_throw,
                wis_saving_throw,
                cha_saving_throw
                FROM characters 
                WHERE id=?1",
        )?;

        let queried_character = stmt.query_row(params![id], |row| {
            Ok(Character {
                id: row.get(0)?,
                name: row.get(1)?,
                alignment: row.get(2)?,
                stats: HashMap::from([
                    ("str".to_string(), row.get(18)?),
                    ("dex".to_string(), row.get(19)?),
                    ("con".to_string(), row.get(20)?),
                    ("int".to_string(), row.get(21)?),
                    ("wis".to_string(), row.get(22)?),
                    ("cha".to_string(), row.get(23)?),
                ]),
                saving_throws: HashMap::from([
                    ("str".to_string(), row.get(24)?),
                    ("dex".to_string(), row.get(25)?),
                    ("con".to_string(), row.get(26)?),
                    ("int".to_string(), row.get(27)?),
                    ("wis".to_string(), row.get(28)?),
                    ("cha".to_string(), row.get(29)?),
                ]),
                proficiency_bonus: row.get(3)?,
                passive_perception: row.get(4)?,
                inspiration: row.get(5)?,
                speed: row.get(6)?,
                gender: row.get(7)?,
                height: row.get(8)?,
                weight: row.get(9)?,
                age: row.get(10)?,
                armor_class: row.get(11)?,
                initiative: row.get(12)?,
                hit_points: row.get(13)?,
                temp_hit_points: row.get(14)?,
                level: row.get(15)?,
                xp: row.get(16)?,
                languages: self.load_characer_languages(row.get(0)?)?,
                proficiencies: self.load_characer_proficiencies(row.get(0)?)?,
                invintory: self.load_character_invintory(row.get(0)?)?
            })
        })?;

        Ok(queried_character)
    }

    // Deletes a SQLite row that matches the id element of the Character
    // struct argument.
    pub fn delete_character(&self, character: &Character) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("DELETE FROM characters WHERE id=?1")?;

        stmt.execute([character.id])?;

        Ok(())
    }

    // Queries the SQLite DB and creates an iterator of all rows in the DB,
    // we then instantiate a Character struct with each row of data and push it
    // to a vector, which is returned.
    // Currently unused; keeping here for any poss features that may utilize all
    // saved characters in the future.
    pub fn get_all_characters(&self) -> Result<Vec<Character>> {
        let mut stmt = self.connection.prepare(
            "SELECT 
            id,
            name,
            alignment,
            proficiency_bonus,
            passive_perception,
            inspiration,
            speed,
            gender,
            height,
            weight,
            age,
            armor_class,
            initiative,
            hit_points,
            temp_hit_points,
            level,
            xp,
            str,
            dex,
            con,
            int,
            wis,
            cha,
            str_saving_throw,
            dex_saving_throw,
            con_saving_throw,
            int_saving_throw,
            wis_saving_throw,
            cha_saving_throw
            FROM characters",
        )?;

        let characters = stmt.query_map([], |row| {
            Ok(Character {
                id: row.get(0)?,
                name: row.get(1)?,
                alignment: row.get(2)?,
                proficiency_bonus: row.get(3)?,
                passive_perception: row.get(4)?,
                inspiration: row.get(5)?,
                speed: row.get(6)?,
                gender: row.get(7)?,
                height: row.get(8)?,
                weight: row.get(9)?,
                age: row.get(10)?,
                armor_class: row.get(11)?,
                initiative: row.get(12)?,
                hit_points: row.get(13)?,
                temp_hit_points: row.get(14)?,
                level: row.get(15)?,
                xp: row.get(16)?,
                stats: HashMap::from([
                    ("str".to_string(), row.get(17)?),
                    ("dex".to_string(), row.get(18)?),
                    ("con".to_string(), row.get(19)?),
                    ("int".to_string(), row.get(20)?),
                    ("wis".to_string(), row.get(21)?),
                    ("cha".to_string(), row.get(22)?),
                ]),
                saving_throws: HashMap::from([
                    ("str".to_string(), row.get(23)?),
                    ("dex".to_string(), row.get(24)?),
                    ("con".to_string(), row.get(25)?),
                    ("int".to_string(), row.get(26)?),
                    ("wis".to_string(), row.get(27)?),
                    ("cha".to_string(), row.get(28)?),
                ]),
                languages: self.load_characer_languages(row.get(0)?)?,
                proficiencies: self.load_characer_proficiencies(row.get(0)?)?,
                invintory: self.load_character_invintory(row.get(0)?)?
            })
        })?;
        characters.into_iter().collect()
    }

    // Queires the SQLite DB and creates an iterator of all rows.
    // Like get_all_characters, a lighter "SavedCharacter" struct is instantiated
    // for row. A Vec of these structs is returned to the caller.
    // This is now called for the SelectScreen of the App to display all saved
    // characters. However, these SavedCharacter structs are much lighter than
    // their Character counterparts, containing only the information displayed to the
    // terminal, and their corresponding ids to load from the DB.
    pub fn list_all_characters(&self) -> Result<Vec<SavedCharacter>> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, name, race, class FROM characters")?;
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

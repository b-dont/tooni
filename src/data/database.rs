use crate::{data::character::SavedCharacter, Character};
use rusqlite::{params, Connection, Result};

use super::language::Language;

// Database interface.
// This struct and its impls represent
// all needed interaction with the SQLite database.
//
// TODO: Consider PRAGMA SQLite statement at connection open
pub struct Database {
    connection: Connection
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
//        self.create_spells_table();
//        self.create_items_table();
//        self.create_backgrounds_table();
//        self.create_classes_table();
//        self.create_races_table();
//        self.create_stats_table();

        Ok(())
    }

//                race INTEGER REFERENCES raceconfigs(id),
//                class INTEGER REFERENCES classconfigs(id),
//                background INTEGER REFERENCES backgroundconfigs(id),
//                stats INTEGER REFERENCES statsconfigs(id),
//                proficiencies INTEGER REFERENCES proficiency_savingthrows_configs(id),
//                equipment !TODO,
//                spells !TODO,


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
                xp INTEGER
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_languages_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS languages (
                id INTEGER PRIMARY KEY, 
                name TEXT UNIQUE NOT NULL,
                description TEXT UNIQUE NOT NULL)",
            []
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS character_languages (
                character INTEGER REFERENCES character(id),
                language INTEGER REFERENCES languages(id),
                PRIMARY KEY (character, language)
            )", 
            []
        )?;
        Ok(())
    }

    pub fn save_character_languages(&self, id: Option<i64>, langs: &Vec<Language>) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO character_languages (
                character,
                language
            )
            VALUES (?1, ?2)"
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
            VALUES (?1, ?2, ?3)"
        )?;

        stmt.execute(params![
            lang.id,
            lang.name,
            lang.description
        ])?;
        Ok(())
    }

    pub fn load_characer_languages(&self, id: Option<i64>) -> Result<Vec<Language>> {
        let mut stmt = self.connection.prepare(
            "SELECT
            language
            FROM character_languages WHERE character=?1
            "
        )?;

        let character_languages = stmt.query_map([id], |row| {
            self.load_language(row.get(0)?)
        })?;

        character_languages.into_iter().collect()

    }

    pub fn load_language(&self, id: i64) -> Result<Language> {
       let mut stmt = self
           .connection
           .prepare("
               SELECT
               id,
               name,
               description
               FROM languages WHERE id=?1
            ")?;

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
        let mut stmt = self
            .connection
            .prepare(
                "SELECT * FROM languages"
            )?;

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
            xp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)"
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
            character.xp
        ])?;

        self.save_character_languages(character.id, &character.languages)?;

        Ok(())
    }

    // Loads a character row from the database that matches the given id.
    // TODO: I think this code will panic if an invalid ID is given;
    // will need to handle this error instead of panicing. This kind of
    // error shouldn't happen, as a user will never call this function with
    // any kind of "custom" id argument.
    pub fn load_character(&self, id: i64) -> Result<Character> {
        let mut stmt = self
            .connection
            .prepare("SELECT 
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
                FROM characters 
                WHERE id=?1")?;

        let queried_character = stmt.query_row(params![id], |row| {
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
                languages: self.load_characer_languages(Some(row.get(0)?))?,
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
    // This is used only for the `select_screen()` function to display all
    // currently saved characters in the database.
    pub fn get_all_characters(&self) -> Result<Vec<Character>> {
        let mut stmt = self.connection.prepare("SELECT 
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
            xp
            FROM characters")?;
        let characters = stmt.query_map([], |row| {
            Ok(Character {
                id: row.get(0)?,
                name: row.get(1)?,
                alignment: row.get(2)?,
                proficiency_bonus: row.get(3)?,
                passive_perception: row.get(4)?,
                inspiration: row.get(5)?,
                languages: self.load_characer_languages(Some(row.get(0)?))?,
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
            })
        })?;
        characters.into_iter().collect()
    }

    pub fn list_all_characters(&self) -> Result<Vec<SavedCharacter>> {
        let mut stmt = self.connection.prepare("SELECT id, name, race, class FROM characters")?;
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

//    pub fn create_proficiencies_savingthrows_table(&self) -> Result<()> {
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS proficiency_savingthrows_configs (
//                id INTEGER PRIMARY KEY,
//                str INTEGER,
//                dex INTEGER,
//                con INTEGER,
//                int INTEGER,
//                wis INTEGER,
//                cha INTEGER,
//                inspiration INTEGER,
//                acrobatics INTEGER,
//                animal_handling INTEGER,
//                arcana INTEGER,
//                athletics INTEGER,
//                deception INTEGER,
//                history INTEGER,
//                insight INTEGER,
//                intimidation INTEGER,
//                investigation INTEGER,
//                medicine INTEGER,
//                nature INTEGER,
//                perception INTEGER,
//                performance INTEGER,
//                persuasion INTEGER,
//                religion INTEGER,
//                sleight_of_hand INTEGER,
//                stealth INTEGER,
//                survival INTEGER
//            )", 
//            []
//        )?;
//
//        Ok(())
//    }
//
//    TODO: This can just be held in the character table
//    pub fn create_stats_table(&self) -> Result<()> {
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS statsconfigs (
//                id INTEGER PRIMARY KEY,
//                str INTEGER,
//                dex INTEGER,
//                con INTEGER,
//                int INTEGER,
//                wis INTEGER,
//                cha INTEGER
//            )", 
//            []
//        )?;
//        Ok(())
//    }
//
//    pub fn create_spells_table (&self) -> Result<()> {
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS spells (
//                name TEXT NOT NULL PRIMARY KEY,
//                school TEXT NOT NULL,
//                level INTEGER,
//                casting_time INTEGER,
//                range INTEGER,
//                components TEXT NOT NULL,
//                duration INTEGER,
//                description TEXT NOT NULL
//            )", 
//            []
//        )?;
//        Ok(())
//    }
//
//    pub fn create_items_table (&self) -> Result<()> {
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS items (
//                name TEXT NOT NULL PRIMARY KEY,
//                class TEXT NOT NULL,
//                cost INTEGER,
//                damage INTEGER,
//                weight INTEGER,
//                properties TEXT NOT NULL
//            )", 
//            []
//        )?;
//        Ok(())
//    }
//
//    pub fn create_backgrounds_table (&self) -> Result<()> {
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS backgrounds (
//                name TEXT NOT NULL PRIMARY KEY,
//                skill_prof TEXT NOT NULL,
//                languages TEXT NOT NULL,
//                starting_equipment TEXT NOT NULL,
//                features TEXT NOT NULL,
//                personality_trait TEXT NOT NULL,
//                ideal TEXT NOT NULL,
//                bond TEXT NOT NULL,
//                flaw TEXT NOT NULL
//            )", 
//            []
//        )?;
//
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS backgroundconfigs (
//                id INTEGER PRIMARY KEY,
//                name TEXT NOT NULL,
//                skill_prof TEXT NOT NULL,
//                languages TEXT NOT NULL,
//                starting_equipment TEXT NOT NULL,
//                features TEXT NOT NULL,
//                personality_trait TEXT NOT NULL,
//                ideal TEXT NOT NULL,
//                bond TEXT NOT NULL,
//                flaw TEXT NOT NULL
//            )", 
//            []
//        )?;
//
//        Ok(())
//    }
//
//    pub fn create_classes_table(&self) -> Result<()> {
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS classes (
//                name TEXT NOT NULL PRIMARY KEY,
//                features TEXT NOT NULL,
//                skill_prof TEXT NOT NULL,
//                armor_prof TEXT NOT NULL,
//                weapon_prof TEXT NOT NULL,
//                tool_prof TEXT NOT NULL,
//                saving_throws TEXT NOT NULL,
//                hit_dice INTEGER,
//                spells_known INTEGER,
//                spell_slots INTEGER,
//                spell_slot_level INTEGER
//            )", 
//            []
//        )?;
//
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS classconfigs (
//                id INTEGER PRIMARY KEY,
//                name TEXT NOT NULL,
//                features TEXT NOT NULL,
//                skill_prof TEXT NOT NULL,
//                armor_prof TEXT NOT NULL,
//                weapon_prof TEXT NOT NULL,
//                tool_prof TEXT NOT NULL,
//                saving_throws TEXT NOT NULL,
//                hit_dice INTEGER,
//                spells_known INTEGER,
//                spell_slots INTEGER,
//                spell_slot_level INTEGER
//            )", 
//            []
//        )?;
//
//        Ok(())
//    }
//
//    pub fn create_races_table(&self) -> Result<()> {
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS races (
//                name TEXT NOT NULL PRIMARY KEY,
//                skill_prof TEXT NOT NULL,
//                armor_prof TEXT NOT NULL,
//                weapon_prof TEXT NOT NULL,
//                features TEXT NOT NULL
//            )", 
//            []
//        )?;
//
//        self.connection.execute(
//            "CREATE TABLE IF NOT EXISTS raceconfigs (
//                id INTEGER PRIMARY KEY,
//                name TEXT NOT NULL,
//                skill_prof TEXT NOT NULL,
//                armor_prof TEXT NOT NULL,
//                weapon_prof TEXT NOT NULL,
//                features TEXT NOT NULL
//            )", 
//            []
//        )?;
//
//        Ok(())
//    }
//
//

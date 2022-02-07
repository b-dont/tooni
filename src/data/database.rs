use super::character::Model;
use crate::{
    data::{
        background::Background,
        character::SavedCharacter,
        spells::Spell,
        tables::{JunctionTable, Table},
    },
    Character,
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
                    {} INTEGER REFERENCES {}, 
                    {} INTEGER REFERENCES {}, 
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
            []
        )?;
        Ok(())
    }

    pub fn delete_row(&self, id: i64, table: Table) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare(format!("DELETE FROM {} WHERE id=?1", table.name()).as_str())?;
        stmt.execute(params![id])?;
        Ok(())
    }

    pub fn save(&self, table: Table, model: &dyn Model) -> Result<()> {
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

    pub fn load(&self, id: i64, table: Table) -> Result<Box<dyn Model>> {
        let mut stmt = self.connection.prepare(
            format!(
                "SELECT {} FROM {} WHERE id=?1",
                table.queries(),
                table.name()
            )
            .as_str(),
        )?;

        let queried_model = stmt.query_row(params![id], |row| Ok(table.create_model(&row)))?;

        queried_model
    }

    pub fn get_all_rows(&self, table: Table) -> Result<Vec<Box<dyn Model>>> {
        let mut stmt = self
            .connection
            .prepare(format!("SELECT {} FROM {}", table.queries(), table.name()).as_str())?;

        let all_models = stmt.query_map([], |row| Ok(table.create_model(&row)?))?;
        all_models.into_iter().collect()
    }

    pub fn create_backgrounds_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS backgrounds (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS background_profs (
                background INTEGER REFERENCES backgrounds(id),
                proficiency INTEGER REFERENCES proficiencies(id),
                PRIMARY KEY (background, proficiency)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS background_langs (
                background INTEGER REFERENCES backgrounds(id),
                language INTEGER REFERENCES languages(id),
                PRIMARY KEY (background, proficiency)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NORT EXISTS background_starting_equipment (
                background INTEGER REFERENCES backgrounds(id),
                item INTEGER REFERENCES items(id),
                PRIMARY KET (background, item)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS backgound_features (
                background INTEGER REFERENCES backgrounds(id),
                feature INTEGER REFERENCES features(id),
                PRIMARY KEY (background, feature)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS background_personality_trait (
                background INTEGER REFERENCES backgrounds(id),
                personality_trait INTEGER REFERENCES personality_traits(id),
                PRIMARY KEY (background, personality_trait)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS personality_trait (
                id INTEGER PRIMARY KEY,
                personality_trait TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS background_ideal (
                background INTEGER REFERENCES backgrounds(id),
                ideal INTEGER REFERENCES ideals(id),
                PRIMARY KEY (background, ideal)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS ideals (
                id INTEGER PRIMARY KEY,
                ideal TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS background_bond (
                background INTEGER REFERENCES backgrounds(id),
                bond INTEGER REFERENCES bonds(id),
                PRIMARY KEY (background, bond)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS bonds (
                id INTEGER PRIMARY KEY,
                bond TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS background_flaw (
                background INTEGER REFERENCES backgrounds(id),
                flaw INTEGER REFERENCES flaws(id),
                PRIMARY KEY (background, flaw)
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS flaws (
                id INTEGER PRIMARY KEY,
                flaw TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn save_backgound(&self, background: Background) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO backgrounds (
                id,
                name
            ) VALUES (?1, ?2)",
        )?;
        stmt.execute(params![background.id, background.name])?;

        Ok(())
    }

    pub fn create_spell_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS spells (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                school TEXT NOT NULL,
                level INTEGER,
                casting_time INTEGER,
                range INTEGER,
                duration INTEGER,
                description TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS character_spells (
                character INTEGER REFERENCES characters(id),
                spell INTEGER REFERENCES spells(id),
                PRIMARY KEY (character, spell)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn save_character_spells(&self, id: Option<i64>, spells: &Vec<Spell>) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO character_spells (
                character,
                spell 
            )
            VALUES (?1, ?2)",
        )?;

        for spell in spells {
            stmt.execute(params![id, spell.id])?;
        }
        Ok(())
    }

    pub fn save_spell(&self, spell: Spell) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO spells (
            id,
            name,
            school,
            level,
            casting_time,
            range,
            components,
            duration,
            description
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
                ?9
                )",
        )?;

        stmt.execute(params![
            spell.id,
            spell.name,
            spell.school,
            spell.level,
            spell.casting_time,
            spell.range,
            spell.components,
            spell.duration,
            spell.description
        ])?;
        Ok(())
    }

    pub fn load_character_spells(&self, id: i64) -> Result<Vec<Spell>> {
        let mut stmt = self.connection.prepare(
            "SELECT
            character,
            spell
            FROM character_spells WHERE character=?1
            ",
        )?;

        let spells = stmt.query_map([id], |row| self.load_spell(row.get(1)?))?;
        spells.into_iter().collect()
    }

    pub fn load_spell(&self, id: i64) -> Result<Spell> {
        let mut stmt = self.connection.prepare(
            "
               SELECT
                id,
                name,
                school,
                level,
                casting_time,
                range,
                components,
                duration,
                description
               FROM spells WHERE id=?1
            ",
        )?;

        let queried_spell = stmt.query_row(params![id], |row| {
            Ok(Spell {
                id: row.get(0)?,
                name: row.get(1)?,
                school: row.get(2)?,
                level: row.get(3)?,
                casting_time: row.get(4)?,
                range: row.get(5)?,
                components: row.get(6)?,
                duration: row.get(7)?,
                description: row.get(8)?,
            })
        })?;

        Ok(queried_spell)
    }

    pub fn get_all_spells(&self) -> Result<Vec<Spell>> {
        let mut stmt = self.connection.prepare("SELECT * FROM spells")?;

        let spells = stmt.query_map([], |row| {
            Ok(Spell {
                id: row.get(0)?,
                name: row.get(1)?,
                school: row.get(2)?,
                level: row.get(3)?,
                casting_time: row.get(4)?,
                range: row.get(5)?,
                components: row.get(6)?,
                duration: row.get(7)?,
                description: row.get(8)?,
            })
        })?;
        spells.into_iter().collect()
    }

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

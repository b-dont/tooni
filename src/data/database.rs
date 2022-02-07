use crate::{
    data::{
        background::Background,
        character::SavedCharacter,
        feature::Feature,
        items::Item,
        language::Language,
        proficiency::Proficiency,
        spells::Spell,
        stats::Stats::{CHA, CON, DEX, INT, STR, WIS},
        tables::{Table, JunctionTable}
    },
    Character,
};
use rusqlite::{params, Result, Connection, params_from_iter};
use std::collections::HashMap;
use super::character::Model;

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
        self.create_item_tables()?;
        self.create_spell_tables()?;
        self.create_backgrounds_tables()?;
        self.create_feature_tables()?;

        Ok(())
    }

    pub fn create_table(&self, table: Table) -> Result<()> {
        self.connection.execute(
            format!("CREATE TABLE IF NOT EXISTS {} ({})", table.name(), table.columns()).as_str(), 
            []
        )?;
        Ok(())
    }

    pub fn save_to_table(&self, table: Table, modle: &dyn Model) -> Result<()> {
        let mut stmt = self.connection.prepare(
            format!("REPLACE INTO {} ({}) VALUES ({})", table.name(), table.table_columns(), table.values()).as_str())?;

        stmt.execute(params_from_iter(modle.parameters().into_iter()))?;
        Ok(())
    }

    pub fn load_from_table(&self, id: i64, table: Table) -> Result<Box<dyn Model>> {
        let mut stmt = self.connection.prepare(
            format!("SELECT ({}) FROM {} WHERE id=?1", table.name(), table.columns()).as_str()
        )?;

        let queried_prof = stmt.query_row(params![id], |row| {
            Ok(table.create_model(&row))
        })?;

        queried_prof
    }

    pub fn get_all_rows(&self, table: Table) -> Result<Vec<Box<dyn Model>>> {
        let mut stmt = self.connection.prepare(
            format!("SELECT ({}) FROM {}", table.table_columns(), table.name()).as_str()
        )?;

        let all_models = stmt.query_map([], |row| {
            Ok(table.create_model(&row)?)
        })?;
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
                str_saving_throw INTEGER,
                dex_saving_throw INTEGER,
                con_saving_throw INTEGER,
                int_saving_throw INTEGER,
                wis_saving_throw INTEGER,
                cha_saving_throw INTEGER
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_feature_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS features (
                id INTEGER PRIMARY KEY,
                class TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            [],
        )?;
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS character_features (
                character INTEGER REFERENCES characters(id),
                feature INTEGER REFERENCES features(id),
                PRIMARY KEY (character, feature)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn save_character_features(&self, id: Option<i64>, features: &Vec<Feature>) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO character_features(
                character,
                feature
            )
            VALUES (?1, ?2)",
        )?;

        for feature in features {
            stmt.execute(params![id, feature.id])?;
        }
        Ok(())
    }

    pub fn save_feature(&self, feature: Feature) -> Result<()> {
        let mut stmt = self.connection.prepare(
            "REPLACE INTO features (
            id,
            class,
            name,
            description
            )
            VALUES (?1, ?2, ?3)",
        )?;

        stmt.execute(params![
            feature.id,
            feature.name,
            feature.class.map_or_else(String::new, |v| v.to_string())
        ])?;
        Ok(())
    }

    pub fn load_characer_features(&self, id: i64) -> Result<Vec<Feature>> {
        let mut stmt = self.connection.prepare(
            "SELECT
            character,
            feature
            FROM character_features WHERE character=?1
            ",
        )?;

        let character_features = stmt.query_map([id], |row| self.load_feature(row.get(1)?))?;
        character_features.into_iter().collect()
    }

    pub fn load_feature(&self, id: i64) -> Result<Feature> {
        let mut stmt = self.connection.prepare(
            "
               SELECT
               id,
               class,
               name,
               description
               FROM features WHERE id=?1
            ",
        )?;

        let queried_feature = stmt.query_row(params![id], |row| {
            Ok(Feature {
                id: row.get(0)?,
                class: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        Ok(queried_feature)
    }

    pub fn get_all_features(&self) -> Result<Vec<Feature>> {
        let mut stmt = self.connection.prepare("SELECT * FROM features")?;

        let features = stmt.query_map([], |row| {
            Ok(Feature {
                id: row.get(0)?,
                class: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
            })
        })?;
        features.into_iter().collect()
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

    pub fn create_item_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                class TEXT NOT NULL,
                quantity INTEGER,
                rarity TEXT NOT NULL,
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

    pub fn save_character_invintory(&self, id: Option<i64>, items: &Vec<Item>) -> Result<()> {
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
            rarity,
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
                ?5, 
                ?6,
                ?7,
                ?8,
                ?9
                )",
        )?;

        stmt.execute(params![
            item.id,
            item.name,
            item.class,
            item.quantity,
            item.rarity.map_or_else(String::new, |v| v.to_string()),
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

        let invintory = stmt.query_map([id], |row| self.load_item(row.get(1)?))?;
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
               rarity,
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
                rarity: row.get(4)?,
                value: row.get(5)?,
                weight: row.get(6)?,
                properties: row.get(7)?,
                description: row.get(8)?,
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
                rarity: row.get(4)?,
                value: row.get(5)?,
                weight: row.get(6)?,
                properties: row.get(7)?,
                description: row.get(8)?,
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

        stmt.execute(params![
            prof.id,
            prof.name,
            prof.class.map_or_else(String::new, |v| v.to_string())
        ])?;
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
                id: row.get("id")?,
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

        let character_languages = stmt.query_map([id], |row| self.load_language(row.get(1)?))?;
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

    pub fn save_character(&self, character: &Character) -> Result<()> {
        let mut stmt = self.connection.prepare(
            // TODO: Change to SQLite UPDATE statement
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
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29
                    )",
        )?;

        stmt.execute(params![
            character.id,
            character.name,
            character.alignment.to_string(),
            character.proficiency_bonus,
            character.passive_perception,
            character.inspiration,
            character.speed,
            character.gender.to_string(),
            character.height,
            character.weight,
            character.age,
            character.armor_class,
            character.initiative,
            character.hit_points,
            character.temp_hit_points,
            character.level,
            character.xp,
            character.stats.get(&STR),
            character.stats.get(&DEX),
            character.stats.get(&CON),
            character.stats.get(&INT),
            character.stats.get(&WIS),
            character.stats.get(&CHA),
            character.saving_throws.get(&STR),
            character.saving_throws.get(&DEX),
            character.saving_throws.get(&CON),
            character.saving_throws.get(&INT),
            character.saving_throws.get(&WIS),
            character.saving_throws.get(&CHA),
        ])?;

        self.save_character_languages(character.id, &character.languages)?;
        self.save_character_proficiencies(character.id, &character.proficiencies)?;
        self.save_character_invintory(character.id, &character.invintory)?;
        self.save_character_spells(character.id, &character.spells)?;
        Ok(())
    }

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
                    (STR, row.get(18)?),
                    (DEX, row.get(19)?),
                    (CON, row.get(20)?),
                    (INT, row.get(21)?),
                    (WIS, row.get(22)?),
                    (CHA, row.get(23)?),
                ]),
                saving_throws: HashMap::from([
                    (STR, row.get(24)?),
                    (DEX, row.get(25)?),
                    (CON, row.get(26)?),
                    (INT, row.get(27)?),
                    (WIS, row.get(28)?),
                    (CHA, row.get(29)?),
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
                invintory: self.load_character_invintory(row.get(0)?)?,
                spells: self.load_character_spells(row.get(0)?)?,
            })
        })?;

        Ok(queried_character)
    }

    pub fn delete_character(&self, character: &Character) -> Result<()> {
        let mut languages_stmt = self
            .connection
            .prepare("DELETE FROM character_languages WHERE character=?1")?;

        let mut proficiencies_stmt = self
            .connection
            .prepare("DELETE FROM character_proficiencies WHERE character=?1")?;

        let mut invintory_stmt = self
            .connection
            .prepare("DELETE FROM character_invintory WHERE character=?1")?;

        let mut character_stmt = self
            .connection
            .prepare("DELETE FROM characters WHERE id=?1")?;

        invintory_stmt.execute([character.id])?;
        proficiencies_stmt.execute([character.id])?;
        languages_stmt.execute([character.id])?;
        character_stmt.execute([character.id])?;

        Ok(())
    }

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
                stats: HashMap::from([
                    (STR, row.get(18)?),
                    (DEX, row.get(19)?),
                    (CON, row.get(20)?),
                    (INT, row.get(21)?),
                    (WIS, row.get(22)?),
                    (CHA, row.get(23)?),
                ]),
                saving_throws: HashMap::from([
                    (STR, row.get(24)?),
                    (DEX, row.get(25)?),
                    (CON, row.get(26)?),
                    (INT, row.get(27)?),
                    (WIS, row.get(28)?),
                    (CHA, row.get(29)?),
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
                invintory: self.load_character_invintory(row.get(0)?)?,
                spells: self.load_character_spells(row.get(0)?)?,
            })
        })?;
        characters.into_iter().collect()
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

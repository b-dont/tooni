    pub fn create_spell_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS spells (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                school TEXT NOT NULL,
                level INTEGER,
                casting_time INTEGER,
                range INTEGER,
                components TEXT NOT NULL,
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
                description: row.get(8)?
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
                description: row.get(8)?
            })
        })?;
        spells.into_iter().collect()
    }


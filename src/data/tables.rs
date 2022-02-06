#[derive(Debug, Clone)]
pub enum Tables {
    CharacterTable,
    CharacterProfs,
    CharacterSpells,
    CharacterLangs,
    CharacterInvintory,
    CharacterFeatures,
//    ProficiencyTable,
//    SpellsTable,
//    LanguagesTable,
//    ItemsTable,
//    FeaturesTable,
}

impl Tables {
    pub fn table_name(&self) -> String {
        match self {
            &Tables::CharacterTable => "characters".to_string(),
            &Tables::CharacterProfs => "character_proficiencies".to_string(),
            &Tables::CharacterSpells => "character_spells".to_string(),
            &Tables::CharacterLangs => "character_languages".to_string(),
            &Tables::CharacterInvintory => "character_invintory".to_string(),
            &Tables::CharacterFeatures => "character_features".to_string(),
        }
    }
}

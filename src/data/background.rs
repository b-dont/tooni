use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Background {
    pub skill_proficiencies: Vec<String>,
    pub languages: Vec<String>,
    pub starting_equipment: Vec<String>,
    pub features: HashMap<String, Vec<String>>,
    pub personality_trait: String,
    pub ideal: String,
    pub bond: String,
    pub flaw: String,
}

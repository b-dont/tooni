use  std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Race {
    pub name: String,
    pub skill_proficiencies: Option<Vec<String>>,
    pub armor_proficiencies: Option<Vec<String>>,
    pub weapon_proficiencies: Option<Vec<String>>,
    pub features: HashMap<String, String>,
}

#[derive(Default, Debug, Clone)]
pub struct Feature {
    id: Option<i64>,
    // TODO: class will need to change to Enum;
    // Background, Race, Class, ect.
    class: String,
    name: String,
    description: String,
}

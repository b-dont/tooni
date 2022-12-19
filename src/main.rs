use crate::data::{
    background::Background,
    character::Character,
    database::Database,
    items::{Item, ItemRarity},
    language::Language,
    proficiency::{Proficiency, ProficiencyClass},
};
use anyhow::Result;
use data::feature::{Feature, FeatureClass};

// use crossterm::{
//     cursor, queue,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use std::io::{stdout, Write};

// mod state;
mod data;

// TODO: This is a mess right now; just ignore all the commented-out code,
// it's for testing.
//
fn main() -> Result<()> {
    //    let mut stdout = stdout();
    //    queue!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0))?;
    //    enable_raw_mode()?;

    let db = Database::new()?;

    let mut test_bg = Background::new();
    test_bg.personality_traits = Some(vec![
        "Test Personality Trait 1".to_string(),
        "Test Personality Trait 2".to_string(),
    ]);
    test_bg.ideals = Some(vec![
        "Test ideal 1".to_string(),
        "Test ideal 2".to_string(),
    ]);

//    let mut test_lang = Language::new();
//    test_lang.description = "Test langage".to_string();
//    test_lang.name = "Testing".to_string();
//
//    let mut test_item = Item::new();
//    test_item.name = "Test Item".to_string();
//    test_item.description = "Testing".to_string();
//    test_item.class = "Test class".to_string();
//    test_item.rarity = Some(ItemRarity::Common);
//
//    let mut test_feature = Feature::new();
//    test_feature.name = "Test feature".to_string();
//    test_feature.description = "Testing".to_string();
//    test_feature.class = Some(FeatureClass::Background);
//
//    db.save(&test_lang)?;
//    db.save(&test_item)?;
//    db.save(&test_feature)?;

    let all_langs = db.get_all_models::<Language>()?;
    let all_items = db.get_all_models::<Item>()?;
    let all_features = db.get_all_models::<Feature>()?;

    test_bg.languages.unwrap().push(all_langs[0].clone());
    test_bg.starting_equipment.unwrap().push(all_items[0].clone());
    test_bg.features.unwrap().as_ref().push(all_features[0].clone());

    db.save::<Background>(&test_bg)?;

    // Instantiate state machine
    //    let mut app = App::new(db)?;

    // Display the first state
    //    app.display_screen()?;

    // This is effectively the main program loop; listens
    // for any user input from crossterm KeyEvent, MouseEvent, or Resize.
    //    app.handle_input()?;

    // If handle_input is broken, we exit the application;
    // disable raw mode and clean up stdout.
    //    disable_raw_mode()?;
    //    queue!(stdout, LeaveAlternateScreen)?;
    //    stdout.flush()?;
    Ok(())
}

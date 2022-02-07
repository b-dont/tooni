use crate::data::{
    character::Character,
    database::Database,
    items::{Item, ItemRarity},
    language::Language,
    proficiency::{Proficiency, ProficiencyClass},
};
use anyhow::Result;
use data::tables::Table;

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
    db.create_table(Table::ProficiencyTable)?;
    let all_profs = db.get_all_rows(Table::ProficiencyTable)?;

    for prof in all_profs {
        println!("{}", prof);
    }
//    let test_armor = Item {
//        id: None,
//        name: "Test Armor".to_string(),
//        class: "Armor".to_string(),
//        quantity: 1,
//        rarity: Some(ItemRarity::Common),
//        value: 2,
//        weight: 10,
//        properties: "Test items don't have properties, silly".to_string(),
//        description: "It's pretty plain".to_string(),
//    };
//
//    let test_weapon = Item {
//        id: None,
//        name: "Test Weapon".to_string(),
//        class: "Weapon".to_string(),
//        quantity: 2,
//        rarity: Some(ItemRarity::Uncommon),
//        value: 4,
//        weight: 12,
//        properties: "Test items don't have properties, silly".to_string(),
//        description: "It's pretty plain".to_string(),
//    };
//
//    let test_potion = Item {
//        id: None,
//        name: "Test Potion".to_string(),
//        class: "Potion".to_string(),
//        quantity: 4,
//        rarity: Some(ItemRarity::Rare),
//        value: 10,
//        weight: 2,
//        properties: "Test items don't have properties, silly".to_string(),
//        description: "It's pretty plain".to_string(),
//    };

//    let medium_armor = Proficiency {
//        id: None,
//        name: Some("Medium Armor".to_string()),
//        class: Some(ProficiencyClass::Armor),
//    };
//
//    let heavy_armor = Proficiency {
//        id: None,
//        name: Some("Heavy Armor".to_string()),
//        class: Some(ProficiencyClass::Armor),
//    };
//
//    let disguise_kit = Proficiency {
//        id: None,
//        name: Some("Disguise Kit".to_string()),
//        class: Some(ProficiencyClass::Tool),
//    };
//
//    let test_tools = Proficiency {
//        id: None,
//        name: Some("Test Tools".to_string()),
//        class: Some(ProficiencyClass::Tool),
//    };
//
//    let acrobatics = Proficiency {
//        id: None,
//        name: Some("Acrobatics".to_string()),
//        class: Some(ProficiencyClass::Skill),
//    };
//
//    let history = Proficiency {
//        id: None,
//        name: Some("History".to_string()),
//        class: Some(ProficiencyClass::Skill),
//    };
//
//    let warhammer = Proficiency {
//        id: None,
//        name: Some("Warhammer".to_string()),
//        class: Some(ProficiencyClass::Weapon),
//    };
//
//    let longbow = Proficiency {
//        id: None,
//        name: Some("Longbow".to_string()),
//        class: Some(ProficiencyClass::Weapon),
//    };
//
//    db.save_to_table(Table::ProficiencyTable, &medium_armor)?;
//    db.save_to_table(Table::ProficiencyTable, &heavy_armor)?;
//    db.save_to_table(Table::ProficiencyTable, &disguise_kit)?;
//    db.save_to_table(Table::ProficiencyTable, &test_tools)?;
//    db.save_to_table(Table::ProficiencyTable, &acrobatics)?;
//    db.save_to_table(Table::ProficiencyTable, &history)?;
//    db.save_to_table(Table::ProficiencyTable, &warhammer)?;
//    db.save_to_table(Table::ProficiencyTable, &longbow)?;

//    let common = Language {
//        id: None,
//        name: "Common".to_string(),
//        description: "It's Common, duh".to_string(),
//    };
//
//    let elvish = Language {
//        id: None,
//        name: "Elvish".to_string(),
//        description: "It's Elvish, duh".to_string(),
//    };
//
//    let orcish = Language {
//        id: None,
//        name: "Orcish".to_string(),
//        description: "It's Orcish, duh".to_string(),
//    };
//
//    let infernal = Language {
//        id: None,
//        name: "Infernal".to_string(),
//        description: "It's Infernal, duh".to_string(),
//    };
//
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

use crate::data::{
    character::Character, database::Database, items::Item, language::Language,
    proficiency::Proficiency,
};
use anyhow::Result;
use crossterm::{
    cursor, queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

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
    db.create_tables()?;

    //    let test_armor = Item {
    //        id: None,
    //        name: "Test Armor".to_string(),
    //        class: "Armor".to_string(),
    //        quantity: 1,
    //        value: 2,
    //        weight: 10,
    //        properties: "Test items don't have properties, silly".to_string(),
    //        description: "It's pretty plain".to_string()
    //    };
    //
    //    let test_weapon = Item {
    //        id: None,
    //        name: "Test Weapon".to_string(),
    //        class: "Weapon".to_string(),
    //        quantity: 2,
    //        value: 4,
    //        weight: 12,
    //        properties: "Test items don't have properties, silly".to_string(),
    //        description: "It's pretty plain".to_string()
    //    };
    //
    //    let test_potion = Item {
    //        id: None,
    //        name: "Test Potion".to_string(),
    //        class: "Potion".to_string(),
    //        quantity: 4,
    //        value: 10,
    //        weight: 2,
    //        properties: "Test items don't have properties, silly".to_string(),
    //        description: "It's pretty plain".to_string()
    //    };
    //
    //    db.save_item(test_armor)?;
    //    db.save_item(test_weapon)?;
    //    db.save_item(test_potion)?;

    //    let medium_armor = Proficiency {
    //        id: None,
    //        name: "Medium Armor".to_string(),
    //        class: "Armor".to_string()
    //    };
    //
    //    let heavy_armor = Proficiency {
    //        id: None,
    //        name: "Heavy Armor".to_string(),
    //        class: "Armor".to_string()
    //    };
    //
    //    let disguise_kit = Proficiency {
    //        id: None,
    //        name: "Disguise Kit".to_string(),
    //        class: "Tools".to_string()
    //    };
    //
    //    let test_tools = Proficiency {
    //        id: None,
    //        name: "Test Tools".to_string(),
    //        class: "Tools".to_string()
    //    };
    //
    //    let acrobatics = Proficiency {
    //        id: None,
    //        name: "Acrobatics".to_string(),
    //        class: "Skill".to_string()
    //    };
    //
    //    let history = Proficiency {
    //        id: None,
    //        name: "History".to_string(),
    //        class: "Skill".to_string()
    //    };
    //
    //    let warhammer = Proficiency {
    //        id: None,
    //        name: "Warhammer".to_string(),
    //        class: "Weapon".to_string()
    //    };
    //
    //    let longbow = Proficiency {
    //        id: None,
    //        name: "Longbow".to_string(),
    //        class: "Weapon".to_string()
    //    };
    //
    //    db.save_proficiency(medium_armor)?;
    //    db.save_proficiency(heavy_armor)?;
    //    db.save_proficiency(disguise_kit)?;
    //    db.save_proficiency(test_tools)?;
    //    db.save_proficiency(acrobatics)?;
    //    db.save_proficiency(history)?;
    //    db.save_proficiency(warhammer)?;
    //    db.save_proficiency(longbow)?;
    //
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
    //    db.save_language(common)?;
    //    db.save_language(elvish)?;
    //    db.save_language(orcish)?;
    //    db.save_language(infernal)?;

    //    let frank = Character::test_character_frank();
    //    let kevin = Character::test_character_kevin();
    //
    //    let all_languages = db.get_all_languages()?;
    //    let all_proficiencies = db.get_all_proficiencies()?;
    // let all_items = db.get_all_items()?;
    //
    //    frank.proficiencies.push(all_proficiencies[0].clone());
    //    frank.proficiencies.push(all_proficiencies[1].clone());
    //    kevin.proficiencies.push(all_proficiencies[2].clone());
    //    kevin.proficiencies.push(all_proficiencies[3].clone());
    //
    //    frank.languages.push(all_languages[0].clone());
    //    frank.languages.push(all_languages[1].clone());
    //    kevin.languages.push(all_languages[2].clone());
    //    kevin.languages.push(all_languages[3].clone());

    //    frank.invintory.push(all_items[0].clone());
    //    frank.invintory.push(all_items[1].clone());
    //    kevin.invintory.push(all_items[2].clone());
    //    kevin.invintory.push(all_items[0].clone());

    //    db.save_character(&frank)?;
    //    db.save_character(&kevin)?;

    //    let all_characters = db.get_all_characters()?;
    //
    //    for character in all_characters {
    //        db.delete_character(&character)?;
    //    }

    //    all_characters[0].proficiencies.push(all_proficiencies[0].clone());
    //    all_characters[0].proficiencies.push(all_proficiencies[1].clone());
    //    all_characters[1].proficiencies.push(all_proficiencies[2].clone());
    //    all_characters[1].proficiencies.push(all_proficiencies[3].clone());
    //
    //    all_characters[0].languages.push(all_languages[0].clone());
    //    all_characters[0].languages.push(all_languages[1].clone());
    //    all_characters[1].languages.push(all_languages[2].clone());
    //    all_characters[1].languages.push(all_languages[3].clone());
    //    all_characters[0].invintory.push(all_items[0].clone());
    //    all_characters[0].invintory.push(all_items[1].clone());
    //    all_characters[1].invintory.push(all_items[2].clone());
    //    all_characters[1].invintory.push(all_items[0].clone());

    //
    //    db.save_character(&all_characters[0].clone())?;
    //    db.save_character(&all_characters[1].clone())?;

    //    for character in &all_characters {
    //        character.print_character();
    //    }

    //    for lang in &all_languages {
    //        println!("{}", lang);
    //    }
    //
    //    for prof in &all_proficiencies {
    //        println!("{}", prof);
    //    }

    //    for prof in &all_characters[0].proficiencies {
    //        println!("{}", prof);
    //    }
    //
    //    for prof in &all_characters[1].proficiencies {
    //        println!("{}", prof);
    //    }
    //
    //    for lang in &all_characters[0].languages {
    //        println!("{}", lang);
    //    }
    //
    //    for lang in &all_characters[1].languages {
    //        println!("{}", lang);
    //    }

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

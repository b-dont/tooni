use crate::{data::{character::Character, database::Database, language::Language}, /* state::app::App*/};
use anyhow::Result;
use crossterm::{
    cursor, queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

// mod state;
mod data;

fn main() -> Result<()> {
//    let mut stdout = stdout();
//    queue!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0))?;
//    enable_raw_mode()?;

    // Instantiate the SQLite database struct
    let db = Database::new()?;
    db.create_tables()?;

//    let mut frank = Character::test_character_frank();
//    let mut kevin = Character::test_character_kevin();

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

//    let all_languages = db.get_all_languages()?;
//
//    frank.languages.push(all_languages[0].clone());
//    frank.languages.push(all_languages[1].clone());
//    kevin.languages.push(all_languages[2].clone());
//    kevin.languages.push(all_languages[3].clone());
//
//    db.save_character(&frank)?;
//    db.save_character(&kevin)?;

    let all_characters = db.get_all_characters()?;

    for character in all_characters {
        println!("{}", character);
    }

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

use crate::{
    character::Character,
    database::Database,
    interface::{get_sheet, select_screen},
};
use anyhow::Result;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
};
use std::io::Stdout;

pub fn handle_keybord_event(event: KeyEvent, mut stdout: &Stdout, db: &Database) -> Result<bool> {
    match event.code {
        // On matching the Esc key, return false to the caller.
        // This will end the main loop and the application.
        KeyCode::Esc => Ok(false),

        // TODO: This needs to change. 'q' will call the `select_screen()`
        // function. If in a character sheet will revert to the select_screen, 
        // or will re-render select_screen if already there. User input needs 
        // to be accounted for differently while in different `screens`.
        KeyCode::Char('q') => {
            select_screen(&stdout, &db)?;
            Ok(true)
        }

        // Currently set to "Vim" key-bindings for `up` and `down` navigation.
        // TODO: Possible feature: user config for key-bindings.
        KeyCode::Char('k') => {
            execute!(stdout, cursor::MoveToPreviousLine(1))?;
            Ok(true)
        }
        KeyCode::Char('j') => {
            execute!(stdout, cursor::MoveToNextLine(1))?;
            Ok(true)
        }

        // TODO: This `Enter` is configured for the `select_screen()`.
        // Another will need to exist in another input loop for the 
        // character sheet.
        //
        // Upon matching `Enter`, we need to account for the cursor's current
        // row position and match this with the corresponding element in the 
        // vector returned by `get_all_characters()`. We then call `load_character()`
        // on that Character struct and return only that character struct.
        //
        // TODO: This seems somewhat inefficent, as we already instantiate the character 
        // struct when we return the vactor of structs. Look into a better way to handle 
        // the struct instead of re-instantiating it.
        KeyCode::Enter => {
            let all_characters = db.get_all_characters()?;
            let current_row = cursor::position()?.1 as u16;
            let all_characters_length = all_characters.len() as u16;
            if current_row == all_characters_length {
                let new_character = Character::new();
                get_sheet(&stdout, &new_character)?;
            } else {
                let selected_character = &all_characters[current_row as usize];
                let loaded_character = db.load_character(selected_character.id.unwrap())?;
                get_sheet(&stdout, &loaded_character)?;
            }
            Ok(true)
        }
        _ => Ok(true),
    }
}

// TODO: Need non-keyboard events
//
// pub fn handle_mouse_event() -> Result<()> {
//     Ok(())
// }
//
// pub fn handle_window_event() -> Result<()> {
//     Ok(())
// }

use crossterm::{
    event::{KeyCode, KeyEvent},
    cursor, execute,
};
use anyhow::Result;
use std::{io::Stdout, convert::TryInto};
use crate::{database::Database, character::Character, interface::get_sheet};

pub fn handle_keybord_event(event: KeyEvent, mut stdout: &Stdout, db: &Database) -> Result<bool> {
    match event.code {
        KeyCode::Esc => Ok(false),
        KeyCode::Up => {
            execute!(stdout, cursor::MoveToPreviousLine(1))?;
            Ok(true)
        }
        KeyCode::Down => {
            execute!(stdout, cursor::MoveToNextLine(1))?;
            Ok(true)
        }
        KeyCode::Enter => {
            let all_characters = db.get_all_characters()?;
            let current_row = cursor::position()?.1 as u16;
            let all_characters_length = all_characters.len() as u16;
            if current_row ==  all_characters_length {
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

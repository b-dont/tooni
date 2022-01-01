use super::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
use crate::character::Character;
use anyhow::Result;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType::All},
};
use std::io::{Stdout, Write};

pub struct SelectScreen {
    saved_characters: Vec<Character>,
}

impl SelectScreen {
    pub fn new(saved_characters: Vec<Character>) -> SelectScreen {
        SelectScreen { saved_characters }
    }
}

impl State for SelectScreen {
    fn display_screen(&self, stdout: &mut Stdout) -> Result<()> {
        execute!(stdout, Clear(All), cursor::MoveTo(0, 0))?;

        for character in &self.saved_characters {
            write!(stdout, "{} {}\r\n", character.name, character.class)?;
            stdout.flush()?;
        }

        write!(stdout, "New Character Sheet..")?;
        stdout.flush()?;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        Ok(())
    }

    fn handle_keyboard_event(
        &self,
        mut stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput> {
        let current_row = cursor::position()?.1 as u16;
        let all_characters_length = self.saved_characters.len() as u16;

        match event.code {
            // On matching the Esc key, return false to the caller.
            // This will end the main loop and the application.
            KeyCode::Esc => Ok(Exit),

            // Currently set to "Vim" key-bindings for `up` and `down` navigation.
            // TODO: Possible feature: user config for key-bindings.
            KeyCode::Char('k') => {
                execute!(stdout, cursor::MoveToPreviousLine(1))?;
                Ok(Input)
            }
            KeyCode::Char('j') => {
                if current_row != all_characters_length {
                    execute!(stdout, cursor::MoveToNextLine(1))?;
                } else {
                }
                Ok(Input)
            }
            KeyCode::Enter => {
                if current_row == all_characters_length {
                    Ok(ChangeState(CharacterScreen(Character::new())))
                } else {
                    let selected_character = &self.saved_characters[current_row as usize];
                    Ok(ChangeState(CharacterScreen(selected_character.clone())))
                }
            }
            _ => Ok(Input),
        }
    }
}

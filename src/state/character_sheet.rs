use super::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
use crate::character::Character;
use anyhow::Result;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub struct CharacterSheet {
    current_character: Character,
}

impl CharacterSheet {
    pub fn new(current_character: Character) -> CharacterSheet {
        let mut state = ListState::default();
        state.select(Some(0));
        CharacterSheet { 
            current_character,
        }
    }
}

impl State for CharacterSheet {
    fn display_screen(&mut self, stdout: &mut Stdout) -> Result<()> {
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(())
    }

    fn handle_keyboard_event(
        &mut self,
        mut stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput> {
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
                execute!(stdout, cursor::MoveToNextLine(1))?;
                Ok(Input)
            }
            KeyCode::Char('q') => Ok(ChangeState(SelectScreen)),
            _ => Ok(Input),
        }
    }
}

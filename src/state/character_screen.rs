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
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub struct CharacterScreen {
    current_character: Character,
}

impl CharacterScreen {
    pub fn new(current_character: Character) -> CharacterScreen {
        CharacterScreen { current_character }
    }
}

impl State for CharacterScreen {
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

    fn display_screen(&mut self, stdout: &mut Stdout) -> Result<()> {
        let backend = CrosstermBackend::new(stdout);
        let character = &self.current_character;
        let character_details = [
            ListItem::new(format!("Name: {}", character.name)),
            ListItem::new(format!("Class: {}", character.class)),
            ListItem::new(format!("Race: {}", character.race)),
        ];

        List::new(character_details)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">");

        Ok(())
    }
}

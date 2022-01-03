use super::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
use crate::character::SavedCharacter;
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
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

pub struct SelectScreen {
    saved_characters: Vec<SavedCharacter>,
    state: ListState
}

impl SelectScreen {
    pub fn new(saved_characters: Vec<SavedCharacter>) -> SelectScreen {
        let mut state = ListState::default();
        state.select(Some(0));
        SelectScreen { 
            saved_characters,
            state
        }
    }
}

impl State for SelectScreen {
    fn display_screen(&mut self, stdout: &mut Stdout) -> Result<()> {
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let all_characters = self
            .saved_characters
            .iter()
            .map(|c| ListItem::new(c.name.clone()))
            .collect::<Vec<_>>();

        terminal.draw(|f| {
            let size = f.size();
            let character_list = List::new(all_characters)
                .block(
                    Block::default()
                        .title("Character Sheets")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">");
            f.render_stateful_widget(character_list, size, &mut self.state)
        })?;
        Ok(())
    }

    fn handle_keyboard_event(
        &mut self,
        mut stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput> {
        let current_row = cursor::position()?.1 as u16;
        let all_characters_length = self.saved_characters.len() as u16;

        match event.code {
            // On matching the Esc key, return false to the caller.
            // This will end the main loop and the application.
            KeyCode::Esc => Ok(Exit),
            KeyCode::Char('k') | KeyCode::Down => {
                self.state.select(self.state.selected().map(|x| x + 1));
                Ok(Input)
            }
            KeyCode::Char('j') | KeyCode::Up => {
                if current_row != all_characters_length {
                    execute!(stdout, cursor::MoveToNextLine(1))?;
                } else {
                }
                Ok(Input)
            }
            KeyCode::Enter => {
                if current_row == all_characters_length {
                    Ok(ChangeState(CharacterScreen(SavedCharacter::new())))
                } else {
                    let selected_character = &self.saved_characters[current_row as usize];
                    Ok(ChangeState(CharacterScreen(selected_character.clone())))
                }
            }
            _ => Ok(Input),
        }
    }
}

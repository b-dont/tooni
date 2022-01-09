use super::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
use crate::character::SavedCharacter;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem, ListState},
    Terminal,
};

pub struct SelectScreen {
    saved_characters: Vec<SavedCharacter>,
    state: ListState,
}

impl SelectScreen {
    pub fn new(saved_characters: Vec<SavedCharacter>) -> SelectScreen {
        let mut state = ListState::default();
        state.select(Some(0));
        SelectScreen {
            saved_characters,
            state,
        }
    }
}

impl State for SelectScreen {
    fn display_screen(&mut self, stdout: &mut Stdout) -> Result<()> {
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let mut selections = self
            .saved_characters
            .iter()
            .map(|c| {
                ListItem::new(format!(
                    "{} {} {}",
                    c.name.as_str(),
                    c.race.as_str(),
                    c.class.as_str()
                ))
            })
            .collect::<Vec<_>>();
        selections.push(ListItem::new("New Character Sheet"));

        terminal.draw(|f| {
            let size = f.size();
            let all_selections = List::new(selections)
                .block(Block::default().title("Character Sheets"))
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">");
            f.render_stateful_widget(all_selections, size, &mut self.state)
        })?;
        Ok(())
    }

    fn handle_keyboard_event(
        &mut self,
        _stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput> {
        let all_characters_length = self.saved_characters.len() + 1;
        match event.code {
            // On matching the Esc key, return false to the caller.
            // This will end the main loop and the application.
            KeyCode::Esc => Ok(Exit),
            KeyCode::Char('j') | KeyCode::Down if all_characters_length > 1 => {
                self.state.select(
                    self.state
                        .selected()
                        .map(|x| (x + 1).clamp(0, all_characters_length - 1)),
                );
                Ok(Input)
            }
            KeyCode::Char('k') | KeyCode::Up if all_characters_length > 1 => {
                self.state.select(self.state.selected().map(|x| {
                    x.checked_sub(1)
                        .unwrap_or(0)
                        .clamp(0, all_characters_length)
                }));
                Ok(Input)
            }
            KeyCode::Enter => {
                if self.state.selected() == Some(all_characters_length - 1) {
                    Ok(ChangeState(CharacterScreen(SavedCharacter::new())))
                } else {
                    Ok(ChangeState(CharacterScreen(
                        self.saved_characters
                            [self.state.selected().unwrap_or(all_characters_length - 1)]
                        .clone(),
                    )))
                }
            }
            _ => Ok(Void),
        }
    }
}

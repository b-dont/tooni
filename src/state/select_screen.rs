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
    // When instantiating a new SelectScreen, a vector of all characters
    // currently saved in the sqlite db is passed.
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
        // Define the backend for our tui terminal and instantiate it
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        // This is a vector of tui-rs ListItems which contain the name,
        // race and class of each character saved in the database.
        // Each of these corresponds with a SavedCharacter struct saved
        // in the saved_characters vector, which have the corresponding
        // id to load the full character struct from the sqlite db.
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

        // Here we add an additional ListItem to select a New or Blank
        // character sheet from the main selection screen
        selections.push(ListItem::new("New Character Sheet"));

        // Call the .draw() method on the terminal instance to format
        // and display the vector of ListItems to the terminal.
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

            // Redering occurs here
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

            // Vim key-binds for up/down navigation as well as arrow-keys. When 'down' is detected,
            // we call the .select() method on the list state, and pass
            // the results from the .selected() method on itself to obtian the
            // next ListItem in the ListState. When 'up' is detected, the
            // opposite occurs.
            //
            // The argument for .selected() is clamped to keep the caller in the
            // bounds of the ListItem vector.
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

            // When enter is detected, we check the saved_characters
            // index that corresponds with the ListState index; if the last
            // index is currently 'selected' then we return a SavedCharacter::new(),
            // or a blank character; else, we return the SavedCharacter at the selected
            // index. The call will then instantiate a full Character struct.
            //
            // This returns a ChangeState to the caller 'App' which handles
            // state changes internally.
            KeyCode::Enter => {
                if self.state.selected() == Some(all_characters_length - 1) {
                    Ok(ChangeState(CharacterSheet(SavedCharacter::new())))
                } else {
                    Ok(ChangeState(CharacterSheet(
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

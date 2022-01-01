use super::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
use crate::character::Character;
use anyhow::{Context, Result};
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub struct CharacterScreen {
    current_character: Option<Character>,
}

impl CharacterScreen {
    pub fn new(current_character: Option<Character>) -> CharacterScreen {
        CharacterScreen { current_character }
    }
}

impl State for CharacterScreen {
    fn display_screen(&self, stdout: &mut Stdout) -> Result<()> {
        let backend = CrosstermBackend::new(stdout);

        // This vector of vectors represents each line of our `Paragraph`,
        // TODO: This method will need to be reviewed; I'm not sure if this
        // is the best way to render the text to the screen.
        let character_text = vec![
            Spans::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(
                    self.current_character
                        .as_ref()
                        .context("No Character")?
                        .name
                        .as_str(),
                ),
            ]),
            Spans::from(vec![
                Span::styled("Class: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(
                    self.current_character
                        .as_ref()
                        .context("No Character")?
                        .class
                        .as_str(),
                ),
            ]),
        ];
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        terminal.set_cursor(0, 0)?;

        // Render the full `sheet`.
        // TODO: This also needs review, as we need to account
        // for user navigation around the sheet and how the user
        // may edit and save character data.
        terminal.draw(|f| {
            let size = f.size();
            let sheet = Paragraph::new(character_text).block(
                Block::default()
                    .borders(Borders::NONE),
            );
            f.render_widget(sheet, size);
        })?;

        Ok(())
    }

    fn handle_keyboard_event(
        &self,
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

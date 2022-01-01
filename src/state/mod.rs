use crate::character::Character;
use anyhow::{Context, Result};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
};
use std::io::{stdout, Stdout, Write};
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use HandleKeyboardInput::*;
use States::*;

mod select_screen;

enum HandleKeyboardInput {
    ChangeState(States),
    Input,
    Exit,
}

enum States {
    SelectScreen,
    CharacterScreen(Character),
}

pub struct Screen {
    saved_characters: Vec<Character>,
    current_character: Option<Character>,
    state: Option<Box<dyn State>>,
    stdout: Stdout,
}

impl Screen {
    pub fn new(saved_characters: Vec<Character>) -> Screen {
        Screen {
            // TODO: have either Screen or SelectScreen own this,
            // and the other hold a reference.
            saved_characters: saved_characters.clone(),
            current_character: None,
            state: Some(Box::new(select_screen::SelectScreen::new(saved_characters))),
            stdout: stdout(),
        }
    }

    fn change_state(&mut self, state: States) -> Result<()> {
        match state {
            SelectScreen => {
                self.state = Some(Box::new(select_screen::SelectScreen::new(
                    self.saved_characters.clone(),
                )))
            }
            CharacterScreen(character) => {
                self.state = Some(Box::new(CharacterScreen {
                    current_character: Some(character.clone()),
                }));
                self.current_character = Some(character.clone())
            }
        }
        Ok(())
    }

    pub fn display_screen(&mut self) -> Result<()> {
        if let Some(state) = &self.state {
            state.display_screen(&mut self.stdout)?;
        }
        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<()> {
        loop {
            self.stdout.flush()?;
            match read()? {
                Event::Key(event) => {
                    if let Some(state) = &self.state {
                        match state.handle_keyboard_event(&mut self.stdout, event)? {
                            Input => {}
                            Exit => break,
                            ChangeState(state) => {
                                self.change_state(state)?;
                                self.display_screen()?;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

trait State {
    fn display_screen(&self, stdout: &mut Stdout) -> Result<()>;
    fn handle_keyboard_event(
        &self,
        stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput>;
}

struct CharacterScreen {
    current_character: Option<Character>,
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
                    .title(self.current_character.as_ref().unwrap().name.as_str())
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

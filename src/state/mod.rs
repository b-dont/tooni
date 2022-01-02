use crate::character::{Character, SavedCharacter};
use anyhow::Result;
use crossterm::event::{read, Event, KeyEvent};
use std::io::{stdout, Stdout, Write};
use HandleKeyboardInput::*;
use States::*;

mod character_screen;
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
    saved_characters: Vec<SavedCharacter>,
    current_character: Option<Character>,
    state: Option<Box<dyn State>>,
    stdout: Stdout,
}

impl Screen {
    pub fn new(saved_characters: Vec<SavedCharacter>) -> Screen {
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
                self.current_character = Some(character.clone());
                self.state = Some(Box::new(character_screen::CharacterScreen::new(
                    self.current_character.clone().unwrap_or(Character::new()),
                )));
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

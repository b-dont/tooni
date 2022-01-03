use crate::{
    character::{Character, SavedCharacter},
    database::Database,
};
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
    CharacterScreen(SavedCharacter),
}

pub struct App {
    saved_characters: Vec<SavedCharacter>,
    current_character: Option<Character>,
    state: Option<Box<dyn State>>,
    stdout: Stdout,
    pub db: Database,
}

impl App {
    pub fn new(db: Database) -> Result<App> {
        let mut not_self = App {
            saved_characters: Vec::new(),
            current_character: None,
            state: None,
            stdout: stdout(),
            db,
        };
        not_self.db.create_character_table()?;
        not_self.saved_characters = not_self.db.list_all_characters()?;
        not_self.state = Some(Box::new(select_screen::SelectScreen::new(
            not_self.saved_characters.clone(),
        )));
        Ok(not_self)
    }

    fn change_state(&mut self, state: States) -> Result<()> {
        match state {
            SelectScreen => {
                self.state = Some(Box::new(select_screen::SelectScreen::new(
                    self.saved_characters.clone(),
                )))
            }
            CharacterScreen(character) => {
                if let Some(id) = character.id {
                    self.current_character = Some(self.db.load_character(id)?);
                } else {
                    self.current_character = Some(Character::new());
                }
                self.state = Some(Box::new(character_screen::CharacterScreen::new(
                    self.current_character.clone().unwrap_or(Character::new()),
                )));
            }
        }
        Ok(())
    }

    pub fn display_screen(&mut self) -> Result<()> {
        if let Some(state) = &mut self.state {
            state.display_screen(&mut self.stdout)?;
        }
        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<()> {
        loop {
            self.stdout.flush()?;
            match read()? {
                Event::Key(event) => {
                    if let Some(state) = &mut self.state {
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
    fn display_screen(&mut self, stdout: &mut Stdout) -> Result<()>;
    fn handle_keyboard_event(
        &mut self,
        stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput>;
}

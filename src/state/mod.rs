use crate::{
    character::{Character, SavedCharacter},
    database::Database,
};
use anyhow::Result;
use crossterm::event::{read, Event, KeyEvent};
use std::io::{stdout, Stdout, Write};
use HandleKeyboardInput::*;
use States::*;

mod character_sheet;
mod select_screen;
mod tabs;

enum HandleKeyboardInput {
    ChangeState(States),
    Input,
    Void,
    Exit,
}

enum States {
    SelectScreen,
    CharacterSheet(SavedCharacter),
}

// All the information needed for any state
// of the application is held in the App.
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

        // Calls on the db are made at instantiation;
        // the .create_character_table() method will
        // never make duplicate tables. It will check
        // for an existing table internally, and if
        // it does not exist, it will create one.
        not_self.db.create_character_table()?;

        // .list_all_characters() returns a Vector of SavedCharacter
        // structs; a lightweight representation of each character saved
        // in the db. It contains the name, race, and class strings of
        // the characters and their corresponding id.
        not_self.saved_characters = not_self.db.list_all_characters()?;

        // The default state when launching the application is SelectScreen,
        // so the App's state is instantiated with a Box<SelectScreen>.
        // The saved_characters vector is then passed as its new() argument.
        not_self.state = Some(Box::new(select_screen::SelectScreen::new(
            not_self.saved_characters.clone(),
        )));
        Ok(not_self)
    }

    fn change_state(&mut self, state: States) -> Result<()> {
        // Match on the enum provided to change_state()
        match state {
            SelectScreen => {
                self.state = Some(Box::new(select_screen::SelectScreen::new(
                    self.saved_characters.clone(),
                )))
            }
            // When changing to the CharacterScreen state, we're provided
            // with a SavedCharacter struct, which contains the id of the
            // corresponding character we're attempting to load.
            CharacterSheet(character) => {
                // If the returned SavedCharacter has an id, we call .load_character()
                // with that id on the db. Else, it's a blank character, so
                // we creadte a new Character struct instead.
                // The result is then set as the current_character.
                if let Some(id) = character.id {
                    self.current_character = Some(self.db.load_character(id)?);
                } else {
                    self.current_character = Some(Character::new());
                }

                // Now we change the current state to CharacterScreen and pass the
                // current_character as its argument.
                //
                // .clone() is called on the current_character struct and unwrap_or
                // to account for the Option. If it's None, then we give it a
                // blank Character struct instead (this scinario should never happen).
                self.state = Some(Box::new(character_sheet::CharacterSheet::new(
                    self.current_character.clone().unwrap_or(Character::new()),
                )?));
            }
        }
        Ok(())
    }

    pub fn display_screen(&mut self) -> Result<()> {
        // Each state has its own display_screen method, which is private.
        // Everything is called through App.
        if let Some(state) = &mut self.state {
            state.display_screen(&mut self.stdout)?;
        }
        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<()> {
        // This is effectively the main program loop. We listen for
        // user input here in the form of crossterm events. Input is classified
        // as Key(KetEvent), Mouse(MouseEvent), and Resize(u16, u16) (for terminal resizing)
        // Like display_screen, there is a method for handling each type
        // of input for each state, which is called.
        //
        // Each handle method returns an enum corresponding to the event type, which
        // is handled by App, rather than the states, as the states do not have access
        // to all the information needed to handle every scinario, but App does.
        loop {
            self.stdout.flush()?;
            match read()? {
                Event::Key(event) => {
                    if let Some(state) = &mut self.state {
                        match state.handle_keyboard_event(&mut self.stdout, event)? {
                            Input => {}
                            Void => {}
                            Exit => break,
                            ChangeState(state) => {
                                self.change_state(state)?;
                            }
                        }
                    }
                }
                _ => {}
            }
            // Display screen is always called after any input is detected and handled
            // to account for any new changes in the display of the state.
            self.display_screen()?;
        }
        Ok(())
    }
}

trait State {
    fn display_screen(&mut self, stdout: &mut Stdout) -> Result<()>;
    fn handle_keyboard_event(
        &mut self,
        _stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput>;
}

use crate::character::Character;
use anyhow::Result;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{Clear, ClearType::All},
};
use std::io::{stdout, Stdout, Write};
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub struct Screen {
    state: Option<Box<dyn State>>,
    //    previous_state: Option<Box<dyn State>>,
    saved_characters: Option<Vec<Character>>,
    current_character: Option<Character>,
    stdout: Stdout,
}

impl Screen {
    pub fn new(saved_characters: Vec<Character>) -> Screen {
        Screen {
            state: Some(Box::new(SelectScreen {
                saved_characters: saved_characters.clone(),
            })),
            //            previous_state: None,
            saved_characters: Some(saved_characters),
            current_character: Some(Character::new()),
            stdout: stdout(),
        }
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
                        if state.handle_keybord_event(&mut self.stdout, event)? {
                        } else {
                            break;
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
    fn handle_keybord_event(&self, stdout: &Stdout, event: KeyEvent) -> Result<bool>;
}

struct SelectScreen {
    saved_characters: Vec<Character>,
}

impl State for SelectScreen {
    fn display_screen(&self, stdout: &mut Stdout) -> Result<()> {
        execute!(stdout, Clear(All), cursor::MoveTo(0, 0))?;

        for character in &self.saved_characters {
            write!(stdout, "{} {}\r\n", character.name, character.class)?;
            stdout.flush()?;
        }

        write!(stdout, "New Character Sheet..")?;
        stdout.flush()?;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        Ok(())
    }

    fn handle_keybord_event(&self, mut stdout: &Stdout, event: KeyEvent) -> Result<bool> {
        let all_characters = &self.saved_characters;
        let current_row = cursor::position()?.1 as u16;
        let all_characters_length = all_characters.len() as u16;

        match event.code {
            // On matching the Esc key, return false to the caller.
            // This will end the main loop and the application.
            KeyCode::Esc => Ok(false),

            // Currently set to "Vim" key-bindings for `up` and `down` navigation.
            // TODO: Possible feature: user config for key-bindings.
            KeyCode::Char('k') => {
                execute!(stdout, cursor::MoveToPreviousLine(1))?;
                Ok(true)
            }
            KeyCode::Char('j') => {
                if current_row != all_characters_length {
                    execute!(stdout, cursor::MoveToNextLine(1))?;
                } else {
                }
                Ok(true)
            }
            KeyCode::Enter => {
                if current_row == all_characters_length {
                } else {
                    let selected_character = &all_characters[current_row as usize];
                }
                Ok(true)
            }
            _ => Ok(true),
        }
    }
}

struct CharacterScreen {
    current_character: Option<Character>,
}

impl State for CharacterScreen {
    fn display_screen(&self, mut stdout: &mut Stdout) -> Result<()> {
        let backend = CrosstermBackend::new(stdout);

        // This vector of vectors represents each line of our `Paragraph`,
        // TODO: This method will need to be reviewed; I'm not sure if this
        // is the best way to render the text to the screen.
        let character_text = vec![
            Spans::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(self.current_character.as_ref().unwrap().name.as_str()),
            ]),
            Spans::from(vec![
                Span::styled("Class: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(self.current_character.as_ref().unwrap().class.as_str()),
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
                    .borders(Borders::ALL),
            );
            f.render_widget(sheet, size);
        })?;

        Ok(())
    }

    fn handle_keybord_event(&self, stdout: &Stdout, event: KeyEvent) -> Result<bool> {
        Ok(true)
    }
}

use crate::{
    character::Character,
    database::Database,
};
use crossterm::{
    cursor,
    execute,
    event::{KeyCode, KeyEvent},
    terminal::{Clear, ClearType::All},
};
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use anyhow::Result;
use std::io::{Stdout, Write};

enum CurrentScreen {
    Select,
    CharacterSheet
}

pub struct Screen {
    current_screen: CurrentScreen,
    current_character: Option<Character>
}

impl Screen {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Select,
            current_character: None
        }
    }

    pub fn display_screen(&self, stdout: &Stdout, db: &Database) -> Result<()> {
        Ok(())
    }

    pub fn handle_input(&self, stdout: &Stdout) -> Result<()> {
        Ok(())
    }
}


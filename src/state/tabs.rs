use super::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
use crate::character::Character;
use anyhow::Result;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Tabs},
    Terminal,
};

#[derive(Default, Clone)]

pub struct TabsState {
    offset: usize,
    selected: Option<usize>
}

impl TabsState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected(&self) -> Result<Option<usize>> {
        Ok(self.selected)
    }

    pub fn select(&mut self, index: Option<usize>) -> Result<()> {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
        Ok(())
    }
}

enum SheetTab {
}

use crate::character::Character;
use core::fmt;
use enum_iterator::IntoEnumIterator;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders},
    Frame,
};

#[derive(Clone, Copy, IntoEnumIterator)]
pub enum CharacterSheetTab {
    Stats,
    Features,
    Spells,
}

impl CharacterSheetTab {
    pub fn get_all_tabs() -> Vec<CharacterSheetTab> {
        CharacterSheetTab::into_enum_iter().collect()
    }

    pub fn get_all_tab_strings() -> Vec<String> {
        let all_tabs = CharacterSheetTab::get_all_tabs();
        let mut all_tabs_strings = Vec::new();
        for tab in all_tabs {
            all_tabs_strings.push(tab.to_string());
        }
        all_tabs_strings
    }

    pub fn display_tab(
        self,
        frame: &mut Frame<CrosstermBackend<&mut Stdout>>,
        area: Rect,
        character: &Character,
    ) {
        match self {
            CharacterSheetTab::Stats => {}
            _ => {}
        }
    }
}

impl std::fmt::Display for CharacterSheetTab {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> std::fmt::Result {
        match self {
            CharacterSheetTab::Stats => write!(formatter, "Stats"),
            CharacterSheetTab::Features => write!(formatter, "Features"),
            CharacterSheetTab::Spells => write!(formatter, "Spells"),
        }
    }
}

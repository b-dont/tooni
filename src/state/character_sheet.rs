use crate::state::app::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
use crate::character::Character;
use anyhow::Result;
use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent},
    execute,
};
use tui::widgets::{Borders, Block};
use enum_iterator::IntoEnumIterator;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Paragraph, Tabs},
    Terminal,
};

#[derive(Clone, Copy, IntoEnumIterator)]
enum CharacterSheetTab {
    Stats,
    Features,
    Spells
}

impl CharacterSheetTab {
    pub fn tab_name(tab: &CharacterSheetTab) -> String {
        match tab {
            &Self::Stats => "Stats".to_string(),
            &Self::Features => "Features".to_string(),
            &Self::Spells => "Spells".to_string()
        }
    }
}

pub struct CharacterSheet {
    current_character: Character,
    current_tab: Option<CharacterSheetTab>,
    index: usize,
    all_tabs: Vec<CharacterSheetTab>,
}

impl CharacterSheet {
    pub fn new(current_character: Character) -> Result<CharacterSheet> {
        Ok(CharacterSheet {
            current_character,
            current_tab: None,
            index: 0,
            all_tabs: CharacterSheetTab::into_enum_iter().collect()
        })
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.all_tabs.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.all_tabs.len() - 1;
        }
    }
}

impl State for CharacterSheet {
    fn display_screen(&mut self, stdout: &mut Stdout) -> Result<()> {
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(5),
                        Constraint::Percentage(85),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let character_details = vec![Spans::from(vec![Span::styled(
                format!(
                    "{} {} {}",
                    self.current_character.name,
                    self.current_character.race,
                    self.current_character.class
                ),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            )])];

            let details = Paragraph::new(character_details)
                .alignment(tui::layout::Alignment::Center)
                .wrap(tui::widgets::Wrap { trim: true });
 
           f.render_widget(details, chunks[0]);

           let tab_titles = vec!["Stats", "Features", "Spells"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();

            let tabs = Tabs::new(tab_titles)
                .select(self.index)
                .style(Style::default().fg(Color::Gray))
                .highlight_style(Style::default().fg(Color::Green))
                .divider("|");

           f.render_widget(tabs, chunks[1]);

           self.current_tab = Some(self.all_tabs[self.index]);
           let current_tab_display = match self.index {
               0 => {},
               1 => {},
               2 => {},
               _ => {}
           };

        })?;
        Ok(())
    }

    fn handle_keyboard_event(
        &mut self,
        _stdout: &Stdout,
        event: KeyEvent,
    ) -> Result<HandleKeyboardInput> {
        match event.code {
            // On matching the Esc key, return false to the caller.
            // This will end the main loop and the application.
            KeyCode::Esc => Ok(Exit),
            KeyCode::Char('q') => Ok(ChangeState(SelectScreen)),
            KeyCode::Tab => {
                self.next();
                Ok(Input)
            },
            KeyCode::BackTab => {
                self.previous();
                Ok(Input)
            },
            _ => Ok(Input),
        }
    }
}

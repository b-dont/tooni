use super::{tabs::TabState, HandleKeyboardInput, HandleKeyboardInput::*, State, States::*};
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

pub struct CharacterSheet {
    current_character: Character,
    tabs: TabState,
}

impl CharacterSheet {
    pub fn new(current_character: Character) -> Result<CharacterSheet> {
        let mut tabs = TabState::default();
        tabs.select(Some(0))?;
        Ok(CharacterSheet {
            current_character,
            tabs 
        })
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
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let tab_titles = ["Deatils", "Features", "Spells"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();

            let all_tabs = Tabs::new(tab_titles)
                .style(Style::default().fg(Color::Gray))
                .highlight_style(Style::default().fg(Color::Green))
                .divider("|");

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
            f.render_widget(all_tabs, chunks[1]);
        })?;
        Ok(())
    }

    fn handle_keyboard_event(
        &mut self,
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

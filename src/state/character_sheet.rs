use crate::{
    Character,
    state::{
    app::{HandleKeyboardInput, HandleKeyboardInput::*, State, States::*},
    tabs::CharacterSheetTab,
}};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Row,
        Block,
        Borders,
        Tabs,
        Table,
        Cell
    },
    Terminal,
};

pub struct CharacterSheet {
    current_character: Character,
    tabs: TabState,
//    current_tab: CharacterSheetTab,
//    index: usize,
//    all_tabs: Vec<CharacterSheetTab>,
}

impl CharacterSheet {
    pub fn new(current_character: Character) -> CharacterSheet {
        CharacterSheet {
            current_character,
            current_tab: CharacterSheetTab::Stats,
            index: 0,
            all_tabs: CharacterSheetTab::get_all_tabs(),
        }
    }

    fn next(&mut self) {
        self.index = (self.index + 1) % self.all_tabs.len();
        self.current_tab = self.all_tabs[self.index];
    }

    fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.all_tabs.len() - 1;
        }
        self.current_tab = self.all_tabs[self.index];
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
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Length(1),
                        Constraint::Percentage(55),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let key_style = Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD);

            let details_table = Table::new(vec![
                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::styled("Name: ", key_style),
                        Span::raw(self.current_character.name.as_str()),
                    ])),
                    Cell::from(Spans::from(vec![
                        Span::styled("Alignment: ", key_style),
                        Span::raw(self.current_character.alignment.as_str()),
                    ])),
                ]),

                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::styled("Race: ", key_style),
                        Span::raw(self.current_character.race.as_str()),
                    ])),
                    Cell::from(Spans::from(vec![
                        Span::styled("Experience: ", key_style),
                        Span::raw(self.current_character.xp.to_string()),
                    ])),
                ]),

                Row::new(vec![
                     Cell::from(Spans::from(vec![
                        Span::styled("Class: ", key_style),
                        Span::raw(self.current_character.class.as_str()),
                    ])),
                ]),

                Row::new(vec![
                    Cell::from(Spans::from(vec![
                        Span::styled("Background: ", key_style),
                        Span::raw(self.current_character.background.as_str()),
                    ])),
                ]),

            ]).block(Block::default())
              .widths(&[Constraint::Percentage(10), Constraint::Percentage(10), Constraint::Percentage(10)]);

            let tab_titles = CharacterSheetTab::get_all_tab_strings()
                .into_iter()
                .map(Spans::from)
                .collect();

            let all_tabs = Tabs::new(tab_titles)
            let tabs = Tabs::new(tab_titles)
                .select(self.index)
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
            let tab_area = Block::default()
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Rounded)
                .style(Style::default());

            f.render_widget(details_table, chunks[0]);
            f.render_widget(tabs, chunks[2]);
            f.render_widget(tab_area, chunks[3]);
            self.current_tab.display_tab(f, chunks[3], &self.current_character);
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
            }
            KeyCode::BackTab => {
                self.previous();
                Ok(Input)
            }
            _ => Ok(Input),
        }
    }
}

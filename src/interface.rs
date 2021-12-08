use crate::{character::Character, database::Database};
use anyhow::Result;
use tui::{Terminal, backend::CrosstermBackend, widgets::{Paragraph, Block, Borders}, text::{Spans, Span}, style::{Style, Modifier}};
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType::All}
};
use std::io::{Stdout, Write};

pub fn select_screen(mut stdout: &Stdout, db: &Database) -> Result<()> {
    execute!(stdout, Clear(All), cursor::MoveTo(0, 0))?;
    let all_characters = db.get_all_characters()?;

    for character in all_characters {
        write!(stdout, "{} {}\r\n", character.name, character.class)?;
        stdout.flush()?;
    }
    write!(stdout, "New Character Sheet..")?;
    stdout.flush()?;
    execute!(stdout, cursor::MoveTo(0, 0))?;
    Ok(())
}

pub fn get_sheet(stdout: &Stdout, character: &Character) -> Result<()> {
    let backend = CrosstermBackend::new(stdout);
    let character_text = vec![
        Spans::from(vec![
            Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&character.name),
        ]),
        Spans::from(vec![
            Span::styled("Class: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&character.class),
        ]),
    ];
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.set_cursor(0, 0)?;

    terminal.draw(|f| {
        let size = f.size();
        let sheet = Paragraph::new(character_text) 
            .block(Block::default().title(character.name.as_str()).borders(Borders::ALL));
        f.render_widget(sheet, size);
    })?;

    Ok(())
}

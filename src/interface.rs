use crate::{character::Character, database::Database};
use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType::All},
};
use std::io::{Stdout, Write};
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

// Displays a list of all Characters currently saved in the SQLite database.
// TODO: The input loops for this "screen" and the actual character sheets
// themselves will need to be different. For example, when enter is pressed
// this function is called, which will cause issues while on the character sheet.
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

// Render the character sheet to the screen. This is incomplete and needs 
// heavy work with formatting and input detection. 
// TODO: All "sections" of the character sheet need to be accounted for,
// and user input, including editing each element of the sheet and its 
// corresponding struct need to be implemented
pub fn get_sheet(stdout: &Stdout, character: &Character) -> Result<()> {
    let backend = CrosstermBackend::new(stdout);

    // This vector of vectors represents each line of our `Paragraph`,
    // TODO: This method will need to be reviewed; I'm not sure if this
    // is the best way to render the text to the screen.
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

    // Render the full `sheet`. 
    // TODO: This also needs review, as we need to account 
    // for user navigation around the sheet and how the user 
    // may edit and save character data. 
    terminal.draw(|f| {
        let size = f.size();
        let sheet = Paragraph::new(character_text).block(
            Block::default()
                .title(character.name.as_str())
                .borders(Borders::ALL),
        );
        f.render_widget(sheet, size);
    })?;

    Ok(())
}

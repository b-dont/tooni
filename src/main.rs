use crate::{database::Database, state::Screen};
use anyhow::Result;
use crossterm::{
    cursor, execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

mod character;
mod database;
mod state;

fn main() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0))?;
    enable_raw_mode()?;

    // Instantiate the SQLite database struct
    let db = Database::new();

    // Instantiate state machine
    let mut screen_state = Screen::new(db.list_all_characters()?);

    // Create our `characters` table if it does not
    // already exist. Additional tables may be built in
    // the future for SRD data and other datasets.
    db.create_character_table()?;
    screen_state.display_screen();
    execute!(stdout, cursor::MoveTo(0, 0))?;

    screen_state.handle_input()?;

    disable_raw_mode()?;
    queue!(stdout, LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

use crate::{character::Character, database::Database, state::App};
use anyhow::Result;
use crossterm::{
    cursor, queue,
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
    let mut app = App::new(db)?;

    // Display the first state
    app.display_screen()?;

    // This is effectively the main program loop; listens
    // for any user input from crossterm KeyEvent, MouseEvent, or Resize.
    app.handle_input()?;

    // If handle_input is broken, we exit the application;
    // disable raw mode and clean up stdout.
    disable_raw_mode()?;
    queue!(stdout, LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

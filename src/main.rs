use crate::{database::Database, input::handle_keybord_event, interface::select_screen};
use anyhow::Result;
use crossterm::{
    cursor,
    event::{read, Event},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

mod character;
mod database;
mod input;
mod interface;
mod state;

fn main() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0))?;
    enable_raw_mode()?;

    // Instantiate the SQLite database struct
    let db = Database::new();
    let mut all_characters = db.get_all_characters()?;

    // Create our `characters` table if it does not
    // already exist. Additional tables may be built in
    // the future for SRD data and other datasets.
    db.create_character_table()?;

    select_screen(&stdout, &all_characters)?;
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // The main program loop only looks for user input.
    // This loop may need to be moved to the interface callers,
    // and additional Key event loops may be needed for
    // different interface functions.
    loop {
        stdout.flush()?;
        match read()? {
            Event::Key(event) => {
                if handle_keybord_event(event, &stdout, &db)? {
                } else {
                    break;
                }
            }
            _ => {}
        }
    }

    // If `handle_keybord_event()` returns `false`, the input
    // loop ends. Raw mode will need to be disabled, and the
    // terminal screen buffer restored.
    disable_raw_mode()?;
    queue!(stdout, LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

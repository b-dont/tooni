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

fn main() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0))?;
    enable_raw_mode()?;

    let db = Database::new();
    db.create_character_table()?;

    select_screen(&stdout, &db)?;
    execute!(stdout, cursor::MoveTo(0, 0))?;

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

    disable_raw_mode()?;
    queue!(stdout, LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

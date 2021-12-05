use crate::{database::Database, input::handle_keybord_event, interface::print_select_screen};
use anyhow::Result;
use character::Character;
use crossterm::{
    cursor,
    event::{read, Event},
    queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

mod character;
mod database;
mod input;
mod interface;

const DATABASEPATH: &str = "characters.sqlite3";

fn main() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, EnterAlternateScreen, cursor::MoveTo(0, 0))?;
    enable_raw_mode()?;

    let db = Database::new(DATABASEPATH);
    db.create_database()?;

    print_select_screen(&stdout, &db)?;

    loop {
        stdout.flush()?;
        match read()? {
            Event::Key(event) => {
                if handle_keybord_event(event, &stdout)? {
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

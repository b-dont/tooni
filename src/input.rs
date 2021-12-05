use crossterm::{
    event::{KeyCode, KeyEvent},
    Result, cursor, execute,
};
use std::io::{Stdout, Write};

pub fn handle_keybord_event(event: KeyEvent, mut stdout: &Stdout) -> Result<bool> {
    match event.code {
        KeyCode::Esc => Ok(false),
        KeyCode::Char(input) => {
            write!(stdout, "{}", input)?;
            Ok(true)
        }
        _ => Ok(true),
    }
}

// pub fn handle_mouse_event() -> Result<()> {
//     Ok(())
// }
//
// pub fn handle_window_event() -> Result<()> {
//     Ok(())
// }

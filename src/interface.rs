use crate::{character::Character, database::Database};
use anyhow::Result;
use crossterm::{
    execute,
    cursor,
    cursor::{CursorShape::Line, SetCursorShape},
    style::{style, Attribute, Color, Stylize}, event::Event,
};
use std::io::{Stdout, Write};

pub fn print_select_screen(mut stdout: &Stdout, db: &Database) -> Result<()> {
    let all_characters = db.get_all_characters()?;

    for character in all_characters {
        write!(stdout, "{} {}\r\n", character.name, character.class)?;
        stdout.flush()?;
    }
    write!(stdout, "New Character Sheet..")?;
    stdout.flush()?;
    Ok(())
}

pub fn get_sheet(mut stdout: &Stdout, character: &Character) -> Result<()> {
    let character_name = style(&character.name)
        .with(Color::Blue)
        .attribute(Attribute::Bold);
    let character_class = style(&character.class)
        .with(Color::Red)
        .attribute(Attribute::Italic);

    write!(stdout, "{}\r\n{}\r\n", character_name, character_class)?;
    Ok(())
}

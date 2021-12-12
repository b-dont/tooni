use crate::character::Character;
use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType::All},
};
use std::io::{stdout, Write};
use tui::{
    backend::CrosstermBackend,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

pub struct Screen {
    state: Option<Box<dyn State>>,
    saved_characters: Option<Vec<Character>>,
    current_character: Option<Character>,
}

impl Screen {
    pub fn new(all_characters: Vec<Character>) -> Screen {
        Screen {
            state: Some(Box::new(SelectScreen { all_characters })),
            saved_characters: Some(all_characters),
            current_character: Some(Character::new()),
        }
    }

    pub fn display_screen(&mut self) {
        if let Some(state) = &self.state {
            state.display_screen();
        }
    }
}

trait State {
    fn display_screen(&self) -> Result<()>;
}

struct SelectScreen {
    all_characters: Vec<Character>,
}

impl State for SelectScreen {
    fn display_screen(&self) -> Result<()> {
        let mut stdout = stdout();
        execute!(stdout, Clear(All), cursor::MoveTo(0, 0))?;

        for character in self.all_characters {
            write!(stdout, "{} {}\r\n", character.name, character.class)?;
            stdout.flush()?;
        }
        write!(stdout, "New Character Sheet..")?;
        stdout.flush()?;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        Ok(())
    }
}

struct CharacterScreen {
    current_character: Character
}

impl State for CharacterScreen {
    fn display_screen(&self) -> Result<()> {
        let mut stdout = stdout();
        let backend = CrosstermBackend::new(stdout);
    
        // This vector of vectors represents each line of our `Paragraph`,
        // TODO: This method will need to be reviewed; I'm not sure if this
        // is the best way to render the text to the screen.
        let character_text = vec![
            Spans::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(self.current_character.name),
            ]),
            Spans::from(vec![
                Span::styled("Class: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(self.current_character.class),
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
                    .title(self.current_character.name.as_str())
                    .borders(Borders::ALL),
            );
            f.render_widget(sheet, size);
        })?;
    
        Ok(())
    }
}

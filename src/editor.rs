use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;
mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cursor_to(0, 0)?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcone()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let (_, rows) = Terminal::size()?;
        for row in 0..rows {
            Terminal::move_cursor_to(0, row)?;
            Terminal::clear_line()?;
            Terminal::print("~")?;
        }

        Ok(())
    }

    fn draw_welcone() -> Result<(), Error> {
        let (cols, rows) = Terminal::size()?;
        let message = "kilo editor -- version 1.0";
        let col_loc = cols.saturating_sub(message.len() as u16) / 2;
        let row_loc = rows / 3;
        Terminal::move_cursor_to(col_loc, row_loc)?;
        Terminal::print(&message)?;
        Ok(())
    }
}

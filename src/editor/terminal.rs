use std::io::{Stdout, stdout};

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

pub struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn initialize(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        self.clear_screen()?;
        self.draw_rows()?;
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        let (_, rows) = size()?;
        for row in 1..rows {
            self.cursor_move_to((0, row))?;
            print!("~");
        }

        self.reset_cursor()?;

        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn reset_cursor(&mut self) -> Result<(), std::io::Error> {
        self.cursor_move_to((0, 0))
    }

    fn cursor_move_to(&mut self, pos: (u16, u16)) -> Result<(), std::io::Error> {
        execute!(self.stdout, MoveTo(pos.0, pos.1))
    }
}

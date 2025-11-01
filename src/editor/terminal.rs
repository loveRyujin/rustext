use std::io::{Stdout, stdout};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

struct Size {
    height: u16,
    width: u16,
}

struct Pos {
    x: u16,
    y: u16,
}

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
        self.reset_cursor()?;
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        let row_height = Self::size()?.height;
        for row in 0..row_height {
            print!("~");
            if row + 1 < row_height {
                print!("\r\n");
            }
        }

        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn hide_cursor(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, Hide)
    }

    pub fn show_cursor(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, Show)
    }

    pub fn reset_cursor(&mut self) -> Result<(), std::io::Error> {
        self.cursor_move_to(Pos{x:0, y:0})
    }

    fn cursor_move_to(&mut self, pos: Pos) -> Result<(), std::io::Error> {
        execute!(self.stdout, MoveTo(pos.x, pos.y))
    }

    fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { height: height, width: width })
    }
}

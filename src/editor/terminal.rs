use std::io::{Write, stdout};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

struct Size {
    height: u16,
    width: u16,
}

struct Pos {
    x: u16,
    y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::reset_cursor()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn print(text: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(text))?;
        Ok(())
    }

    pub fn draw_rows() -> Result<(), std::io::Error> {
        let row_height = Self::size()?.height;
        for row in 0..row_height {
            Self::clear_line()?;
            Self::print("~")?;
            if row + 1 < row_height {
                Self::print("\r\n")?;
            }
        }

        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn reset_cursor() -> Result<(), std::io::Error> {
        Self::cursor_move_to(Pos { x: 0, y: 0 })?;
        Ok(())
    }

    pub fn show_logo() -> Result<(), std::io::Error> {
        let size = Self::size()?;
        let logo_position = Pos {
            x: size.width / 2,
            y: size.height / 3 * 2,
        };
        Self::cursor_move_to(logo_position)?;
        Self::print("rustext v0.1.0")?;

        Ok(())
    }

    fn cursor_move_to(pos: Pos) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size {
            height: height,
            width: width,
        })
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
}

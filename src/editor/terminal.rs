use std::fmt::Display;
use std::io::{Error, Write, stdout};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::{Command, queue};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Pos {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::reset_cursor()?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn print<T: Display>(text: T) -> Result<(), Error> {
        Self::queue_command(Print(text))?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn reset_cursor() -> Result<(), Error> {
        Self::cursor_move_to(Pos { x: 0, y: 0 })?;
        Ok(())
    }

    pub fn cursor_move_to(pos: Pos) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size {
            height: height,
            width: width,
        })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}

use super::terminal::Terminal;
use std::fs;
use std::io::Error;

mod buffer;
use buffer::Buffer;

use super::Size;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buf: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        if self.buf.is_empty() {
            Self::render_welcome_screen()
        } else {
            self.render_buffer()
        }
    }

    fn render_welcome_screen() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_height in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_height == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empth_row()?;
            }

            if current_height.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_height in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buf.lines.get(current_height) {
                Terminal::print(line)?;
            } else {
                Self::draw_empth_row()?;
            }

            if current_height.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }

        Ok(())
    }

    pub fn load(&mut self, filename: &str) -> Result<(), Error> {
        let file_contents = fs::read_to_string(filename)?;
        for line in file_contents.lines() {
            self.buf.lines.push(line.to_string());
        }

        Ok(())
    }

    fn draw_empth_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor --version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let message_len = welcome_message.len();

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(message_len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::print(welcome_message.as_str())?;

        Ok(())
    }
}

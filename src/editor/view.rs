use super::terminal::Terminal;
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    pub fn render() -> Result<(), Error> {
        let size = Terminal::size()?;
        Terminal::print("Hello, World!\r\n")?;

        for current_height in 1..size.height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_height == size.height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empth_row()?;
            }

            if current_height.saturating_add(1) < size.height {
                Terminal::print("\r\n")?;
            }
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

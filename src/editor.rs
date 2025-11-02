use crossterm::event::Event;
use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;

mod terminal;
use terminal::Terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { should_exit: false }
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
            if self.should_exit {
                break;
            }
            let event = read()?;
            self.eval_event(&event)?;
        }

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = event
        {
            Terminal::print(
                format!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                )
                .as_str(),
            )?;
            println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_exit = true;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_exit {
            Terminal::clear_screen()?;
            Terminal::reset_cursor()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::reset_cursor()?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let size = Terminal::size()?;
        for current_height in 0..size.height {
            Terminal::clear_line()?;
            if current_height == size.height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empth_row()?;
            }

            if current_height + 1 < size.height {
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
        let padding = (width - message_len) / 2;
        let spaces = " ".repeat(padding - 1);

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        Terminal::print(welcome_message.as_str())?;

        Ok(())
    }
}

use core::cmp::min;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use std::io::Error;

mod terminal;
use terminal::Terminal;

use crate::editor::terminal::{Pos, Size};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct Editor {
    should_exit: bool,
    location: Location,
}

impl Editor {
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

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(y.saturating_add(1), height.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(x.saturating_add(1), width.saturating_add(1));
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_add(1);
            }
            _ => (),
        }
        self.location = Location { x, y };

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Event::Key(KeyEvent {
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

            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_exit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
            Terminal::execute()?;
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Pos::default())?;

        if self.should_exit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(Pos {
                col: self.location.x,
                row: self.location.y,
            })?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let size = Terminal::size()?;
        for current_height in 0..size.height {
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

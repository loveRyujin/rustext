use crossterm::cursor::MoveTo;
use crossterm::event::Event;
use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::stdout;

pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor { should_exit: false }
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::draw_rows()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        let (_, rows) = size()?;
        for row in 1..rows {
            execute!(stdout, MoveTo(0, row))?;
            print!("{}", "~");
        }

        Self::reset_cursor()?;
        Ok(())
    }

    fn reset_cursor() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, MoveTo(0, 0))
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.eval_event(&event);
            self.refresh_screen()?;
            if self.should_exit {
                break;
            }
        }

        Ok(())
    }

    fn eval_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = event
        {
            println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_exit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_exit {
            Self::clear_screen()?;
            Self::reset_cursor()?;
            print!("Goodbye!\r\n");
        }

        Ok(())
    }
}

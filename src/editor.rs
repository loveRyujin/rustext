use crossterm::event::Event;
use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
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
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
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
            print!("Goodbye!\r\n");
        }

        Ok(())
    }
}

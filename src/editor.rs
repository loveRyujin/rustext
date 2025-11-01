use crossterm::event::Event;
use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

mod terminal;
use terminal::Terminal;

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

    fn repl(&mut self) -> Result<(), std::io::Error> {
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

    fn eval_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
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

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;

        if self.should_exit {
            Terminal::clear_screen()?;
            Terminal::reset_cursor()?;
            Terminal::print("Goodbye!\r\n")?;
        } else {
            Terminal::draw_rows()?;
            Terminal::reset_cursor()?;
        }

        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }
}

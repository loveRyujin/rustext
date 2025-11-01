use crossterm::event::Event;
use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_exit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_exit: false,
            terminal: Terminal::new(),
        }
    }

    pub fn run(&mut self) {
        self.terminal.initialize().unwrap();
        let result = self.repl();
        self.terminal.terminate().unwrap();
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

    fn eval_event(&mut self, event: &Event) -> Result<(), std::io::Error>{
        if let Key(KeyEvent {
            code,
            modifiers,
            kind,
            state,
        }) = event
        {
            self.terminal.print(format!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r").as_str())?;
            println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_exit = true;
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.terminal.hide_cursor()?;

        if self.should_exit {
            self.terminal.clear_screen()?;
            self.terminal.reset_cursor()?;
            self.terminal.print("Goodbye!\r\n")?;
        } else {
            self.terminal.draw_rows()?;
            self.terminal.reset_cursor()?;
        }

        self.terminal.show_cursor()?;
        self.terminal.execute()?;

        Ok(())
    }
}

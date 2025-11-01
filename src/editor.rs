use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor { should_exit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }

        print!("Goodbye.\r\n");
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;

        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_exit = true;
                    }
                    _ => (),
                }
            }
            if self.should_exit {
                break;
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}

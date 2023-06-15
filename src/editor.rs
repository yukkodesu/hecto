use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{Clear, ClearType}, style::Print,
};
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }
            if self.should_quit {
                break;
            }
            if let Err(e) = self.process_key() {
                die(e);
            }
        }
    }
    fn process_key(&mut self) -> Result<(), std::io::Error> {
        let key_event = read_key()?;
        // execute!(stdout(), Print(format!("{:?}\r\n", key_event)))?;
        match key_event {
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char('q'),
                ..
            } => {
                self.should_quit = true;
            }
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(1, 1))?;
        if self.should_quit {
            execute!(stdout(), Print("Hecto Exit!\r\n"))?;
        }
        Ok(())
    }
}
fn die(e: std::io::Error) {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    panic!("{}", e);
}

fn read_key() -> Result<KeyEvent, std::io::Error> {
    loop {
        match read() {
            Ok(event) => match event {
                Event::Key(event) => {
                    return Ok(event);
                }
                _ => continue,
            },
            Err(e) => return Err(e),
        }
    }
}

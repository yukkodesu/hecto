use crate::terminal::Terminal;
use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal"),
        }
    }
    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                die(e);
            }
            if self.should_quit {
                break;
            } else {
                self.draw_rows();
                execute!(stdout(), MoveTo(0, 0)).unwrap();
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
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        if self.should_quit {
            execute!(stdout(), Print("Hecto Exit!\r\n"))?;
        }
        Ok(())
    }
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            execute!(stdout(), Print("~\r\n")).unwrap();
        }
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

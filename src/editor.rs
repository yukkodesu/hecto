use crate::Terminal;
use std::io::stdout;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
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
            if let Err(e) = Terminal::refresh_screen() {
                die(e);
            }
            if self.should_quit {
                execute!(stdout(), Print("Hecto Exit!\r\n")).unwrap();
                break;
            } else {
                self.draw_rows();
                Terminal::move_cursor(0, 0);
            }
            if let Err(e) = self.process_key() {
                die(e);
            }
        }
    }
    fn process_key(&mut self) -> Result<(), std::io::Error> {
        let key_event = Terminal::read_key()?;
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
    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height - 1 {
            execute!(stdout(), Print("~\r\n")).unwrap();
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::refresh_screen().unwrap();
    panic!("{}", e);
}

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
            if let Err(e) = self.refresh_screen() {
                die(e);
            }
            if self.should_quit {
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
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::move_cursor(0, 0);
        if self.should_quit {
            execute!(stdout(), Print("Hecto Exit!\r\n")).unwrap();
        } else {
            self.draw_rows();
            Terminal::move_cursor(0, 0);
        }
        Terminal::cursor_show();
        Ok(())
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

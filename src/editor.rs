use crate::Terminal;
use std::{fmt::format, io::stdout};

const VERSION: &str = env!("CARGO_PKG_VERSION");

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    queue,
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
            }
            if let Err(e) = self.process_key() {
                die(e);
            }
            Terminal::cursor_show();
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

    fn draw_welcome_msg(&self) {
        let welcome_msg = format!("Hecto editor -- version {}", VERSION);
        let width = (self.terminal.size().width as usize).min(welcome_msg.len());
        let pad = (self.terminal.size().width as usize - width) / 2;
        let welcome_msg = format!("~{}{}\r\n", " ".repeat(pad), &welcome_msg[..width]);
        queue!(stdout(), Print(&welcome_msg)).unwrap();
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_msg();
            } else {
                queue!(stdout(), Print("~\r\n")).unwrap();
            }
        }
        Terminal::flush();
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::move_cursor(0, 0);
        // Terminal::clear_screen();
        if self.should_quit {
            Terminal::clear_screen();
            queue!(stdout(), Print("Hecto Exit!\r\n")).unwrap();
        } else {
            self.draw_rows();
            Terminal::move_cursor(0, 0);
        }
        Terminal::cursor_show();
        Terminal::flush();
        Ok(())
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

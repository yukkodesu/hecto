use crate::{document, Document, Row, Terminal};
use std::{env, io::stdout};

const VERSION: &str = env!("CARGO_PKG_VERSION");

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    queue,
    style::Print,
};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_pos: Position,
    document: Document,
}

impl Editor {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if let Some(filename) = args.get(1) {
            match Document::open(filename) {
                Ok(document) => document,
                Err(e) => {
                    die(e);
                    Document::default()
                }
            }
        } else {
            Document::default()
        };
        Editor {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal"),
            cursor_pos: Position::default(),
            document,
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
        }
    }

    fn process_key(&mut self) -> Result<(), std::io::Error> {
        let key_event = Terminal::read_key()?;
        // println!("{:?}\r\n", key_event);
        match key_event {
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char('q'),
                ..
            } => {
                self.should_quit = true; //Quit on Ctrl+Q
            }

            //Handle arrow key
            KeyEvent {
                code: KeyCode::Left,
                ..
            }
            | KeyEvent {
                code: KeyCode::Right,
                ..
            }
            | KeyEvent {
                code: KeyCode::Up, ..
            }
            | KeyEvent {
                code: KeyCode::Down,
                ..
            }
            | KeyEvent {
                code: KeyCode::Home,
                ..
            }
            | KeyEvent {
                code: KeyCode::End, ..
            }
            | KeyEvent {
                code: KeyCode::PageDown,
                ..
            }
            | KeyEvent {
                code: KeyCode::PageUp,
                ..
            } => self.move_cursor(key_event),

            //Others
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key_event: KeyEvent) {
        let Position { mut x, mut y } = self.cursor_pos;
        let height = self.terminal.size().height as usize;
        let width = self.terminal.size().width as usize;
        match key_event.code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        self.cursor_pos = Position { x, y };
    }

    fn draw_welcome_msg(&self) {
        let welcome_msg = format!("Hecto editor -- version {}", VERSION);
        let width = (self.terminal.size().width as usize).min(welcome_msg.len());
        let pad = (self.terminal.size().width as usize - width) / 2;
        let welcome_msg = format!("~{}{}\r\n", " ".repeat(pad), &welcome_msg[..width]);
        queue!(stdout(), Print(&welcome_msg)).unwrap();
    }

    fn draw_row(&self, row: &Row) {
        let end = self.terminal.size().width as usize;
        let row = row.render(0, end);
        queue!(stdout(), Print(format!("{}\r\n", row))).unwrap();
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row_idx in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row_idx as usize) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row_idx == height / 3 {
                self.draw_welcome_msg();
            } else {
                queue!(stdout(), Print("~\r\n")).unwrap();
            }
        }
        Terminal::flush();
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::move_cursor(&Position::default());
        // Terminal::clear_screen();
        if self.should_quit {
            Terminal::clear_screen();
            queue!(stdout(), Print("Hecto Exit!\r\n")).unwrap();
        } else {
            self.draw_rows();
            Terminal::move_cursor(&self.cursor_pos);
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

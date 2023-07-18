use crate::{Document, Row, Terminal};
use crossterm::style::Color;
use std::{
    env,
    io::stdout,
    time::{Duration, Instant},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 0,
    g: 181,
    b: 173,
};

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

struct StatusMessage {
    text: String,
    time: Instant,
}

impl From<&String> for StatusMessage {
    fn from(value: &String) -> Self {
        Self {
            text: String::from(value),
            time: Instant::now(),
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_pos: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
}

impl Editor {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-Q to quit");
        let document = if let Some(filename) = args.get(1) {
            match Document::open(filename) {
                Ok(document) => document,
                Err(_) => {
                    initial_status = format!("ERR: Could not open file: {}", filename);
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
            offset: Position::default(),
            status_message: StatusMessage::from(&initial_status),
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
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_pos;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key_event: KeyEvent) {
        let get_current_width = |y| {
            if let Some(row) = self.document.row(y) {
                row.len()
            } else {
                0
            }
        };
        let Position { mut x, mut y } = self.cursor_pos;
        let height = self.document.len();
        let mut width = get_current_width(y);
        match key_event.code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = get_current_width(y);
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => y = y.saturating_sub(self.terminal.size().height as usize),
            KeyCode::PageDown => {
                y = y
                    .saturating_add(self.terminal.size().height as usize)
                    .min(height)
            }
            KeyCode::Home => {
                let len_no_whitespace = if let Some(row) = self.document.row(y) {
                    row.len_no_whitespace()
                } else {
                    0
                };
                x = width - len_no_whitespace;
            }
            KeyCode::End => x = width,
            _ => (),
        }
        width = get_current_width(y);
        x = x.min(width);
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
        // let start = 0;
        // let end = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + self.terminal.size().width as usize;
        let row = row.render(start, end);
        queue!(stdout(), Print(format!("{}\r\n", row))).unwrap();
    }
    /* Lorem ipsum dolor sit amet consectetur adipisicing elit. Mollitia dicta doloribus at, ab, laudantium eius officia nihil accusantium non unde quidem culpa repellendus? Inventore ea ex voluptates saepe, alias laboriosam quia aut, incidunt id rem mollitia beatae, rerum nulla eveniet ipsam exercitationem. Magni repellendus temporibus natus maxime, quos qui perspiciatis!  */
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row_idx in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row_idx as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row_idx == height / 3 {
                self.draw_welcome_msg();
            } else {
                queue!(stdout(), Print("~\r\n")).unwrap();
            }
        }
        Terminal::flush();
    }

    fn draw_status_bar(&self) {
        let width = self.terminal.size().width as usize;
        let mut file_name = String::new();
        if let Some(filename) = &self.document.file_name {
            file_name.push_str(filename);
            file_name.truncate(20);
        }
        let mut status = format!("{} - {} lines", file_name, self.document.len());
        let line_indicator = format!(
            "{}:{}",
            self.cursor_pos.y.saturating_add(1),
            self.cursor_pos.x.saturating_add(1)
        );
        if width > status.len() + line_indicator.len() {
            status.push_str(&" ".repeat(width - status.len() - line_indicator.len()));
        }
        status.push_str(&line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        queue!(stdout(), Print(format!("{}\r\n", status))).unwrap();
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            queue!(stdout(), Print(text)).unwrap();
        }
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
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::move_cursor(&Position {
                x: self.cursor_pos.x.saturating_sub(self.offset.x),
                y: self.cursor_pos.y.saturating_sub(self.offset.y),
            });
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

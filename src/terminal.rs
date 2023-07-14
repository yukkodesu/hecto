use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyEvent},
    queue,
    style::{Color, SetBackgroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{stdout, Write};

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let sz = terminal::size()?;
        Ok(Terminal {
            size: Size {
                width: sz.0,
                height: sz.1.saturating_sub(2),
            },
        })
    }

    #[inline]
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        queue!(stdout(), Clear(ClearType::All)).unwrap();
    }
    pub fn clear_current_line() {
        queue!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
    }

    pub fn set_bg_color(color: Color) {
        queue!(stdout(), SetBackgroundColor(color)).unwrap();
    }
    pub fn reset_bg_color() {
        queue!(stdout(), SetBackgroundColor(Color::Reset)).unwrap();
    }

    pub fn move_cursor(pos: &Position) {
        queue!(stdout(), MoveTo(pos.x as u16, pos.y as u16)).unwrap();
    }

    pub fn read_key() -> Result<KeyEvent, std::io::Error> {
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
    pub fn cursor_hide() {
        queue!(stdout(), cursor::Hide {}).unwrap();
    }
    pub fn cursor_show() {
        queue!(stdout(), cursor::Show {}).unwrap();
    }
    pub fn flush() {
        stdout().flush().unwrap();
    }
}

use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
};
use std::io::stdout;

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
                height: sz.1,
            },
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
    pub fn move_cursor(x: u16, y: u16) {
        execute!(stdout(), MoveTo(x, y)).unwrap();
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
        execute!(stdout(), cursor::Hide {}).unwrap();
    }
    pub fn cursor_show() {
        execute!(stdout(), cursor::Show {}).unwrap();
    }
}

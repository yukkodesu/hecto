use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{self, Clear, ClearType}, event::{KeyEvent, Event, read},
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
    pub fn refresh_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
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
}

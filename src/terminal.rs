use crossterm::terminal;

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
}

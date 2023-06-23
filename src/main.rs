mod document;
mod editor;
mod row;
mod terminal;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    enable_raw_mode().unwrap();
    Editor::new().run();
    disable_raw_mode().unwrap();
}

mod editor;
mod terminal;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use editor::Editor;

fn main() {
    enable_raw_mode().unwrap();
    Editor::new().run();
    disable_raw_mode().unwrap();
}

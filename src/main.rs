use crossterm::event::{read, Event, KeyCode, KeyModifiers, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    // println!("Hello, world!");
    enable_raw_mode().unwrap();
    loop {
        match read() {
            Ok(event) => match event {
                Event::Key(event) => {
                    println!("{:?}\r", event);
                    if event == KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL)
                    {
                        break;
                    }
                }
                _ => (),
            },
            Err(e) => die(e),
        }
    }
    disable_raw_mode().unwrap();
}

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    // println!("Hello, world!");
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:#b} ({})\r", b, c);
                }
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(e) => die(e),
        }
    }
    disable_raw_mode().unwrap();
}

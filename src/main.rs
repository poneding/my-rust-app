// use termion::terminal_size;

use std::io::stdout;

use termion::{async_stdin, raw::IntoRawMode};

fn main() {
    println!("name: {}", env!("CARGO_PKG_NAME"));
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    // let (w, h) = terminal_size().unwrap();
    let (w, h) = crossterm::terminal::size().unwrap();
    println!("width: {}", w);
    println!("height: {}", h);

    stdout().lock().into_raw_mode().unwrap();

    let stdin = async_stdin();
}

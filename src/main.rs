use termion::terminal_size;

fn main() {
    println!("name: {}", env!("CARGO_PKG_NAME"));
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    let (w, h) = terminal_size().unwrap();
    println!("width: {}", w);
    println!("height: {}", h);
}

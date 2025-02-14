fn main() {
    println!("name: {}", env!("CARGO_PKG_NAME"));
    println!("version: {}", env!("CARGO_PKG_VERSION"));
}

use std::io;

const MEOW: &str = include_str!("./kitty.txt");

#[inline]
pub fn run() -> io::Result<()> {
    println!("{MEOW}");
    Ok(())
}
